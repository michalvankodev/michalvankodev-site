/*!
Central image generation queue.

Replaces the previous rayon-based parallelization with a tokio-based dispatcher:

- `enqueue_image_job()` is sync and callable from anywhere (askama filters, templates,
  tests) — it just sends on an unbounded channel.
- A single dispatcher actor (on a dedicated leaked `current_thread` runtime driven by
  its own thread) consumes the channel and:
    1. skips jobs whose outputs all already exist (warm cache — no decode at all),
    2. drops duplicates of jobs that are already queued/running,
    3. acquires a semaphore permit and runs the CPU-bound work on `spawn_blocking`.
- The number of semaphore permits bounds the number of busy cores:
  `IMAGE_WORKERS` env var, default `(available_parallelism() - 2).clamp(1, ..)`.
- `wait_until_idle()` resolves once nothing is queued or running — used by the
  `/export-wait` endpoint to make SSG exports deterministic.
*/

use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use tokio::{
    runtime::{Builder, Runtime},
    sync::{mpsc, watch, Semaphore},
};
use tracing::{debug, error, info};

use super::{
    export_format::ExportFormat, image_generator::get_save_path,
};

/// One unit of image generation work: all `formats × resolutions` variants of a
/// single source image, written under `generated_base_path`.
#[derive(Debug, Clone)]
pub struct ImageJob {
    /// Path to the original image on disk (e.g. `static/images/uploads/x.jpg`).
    pub disk_image_path: PathBuf,
    /// Base path (no extension) for generated files,
    /// e.g. `/generated_images/images/uploads/x` or `..._og`.
    pub generated_base_path: PathBuf,
    /// (width, height, pixel_density) variants to generate. The density is only
    /// metadata for srcset markup — it does not influence the generated files.
    pub resolutions: Vec<(u32, u32, f32)>,
    pub formats: Vec<ExportFormat>,
}

impl ImageJob {
    /// Identity used for duplicate detection. Densities are ignored because they
    /// don't affect the output files.
    fn key(&self) -> JobKey {
        JobKey {
            disk_image_path: self.disk_image_path.clone(),
            generated_base_path: self.generated_base_path.clone(),
            resolutions: self
                .resolutions
                .iter()
                .map(|(width, height, _)| (*width, *height))
                .collect(),
            formats: self.formats.clone(),
        }
    }

    /// All final file paths this job would write.
    pub fn output_paths(&self) -> Vec<PathBuf> {
        self.formats
            .iter()
            .flat_map(|format| {
                self.resolutions
                    .iter()
                    .map(move |(width, height, _)| {
                        get_save_path(&self.generated_base_path, *width, *height, *format)
                    })
            })
            .collect()
    }

    /// Warm-cache check: everything this job would produce already exists.
    fn all_outputs_exist(&self) -> bool {
        self.output_paths().iter().all(|path| path.exists())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct JobKey {
    disk_image_path: PathBuf,
    generated_base_path: PathBuf,
    resolutions: Vec<(u32, u32)>,
    formats: Vec<ExportFormat>,
}

struct QueueState {
    /// Jobs currently queued (handed to dispatcher) or running.
    in_flight: Mutex<HashSet<JobKey>>,
    /// pending = enqueued but not yet finished (queued, skipped or running).
    /// Watch channel so `wait_until_idle` can subscribe from any runtime.
    pending: watch::Sender<u64>,
}

impl QueueState {
    fn job_enqueued(&self) {
        self.pending.send_modify(|pending| *pending += 1);
    }

    fn job_finished(&self) {
        self.pending.send_modify(|pending| *pending = pending.saturating_sub(1));
    }

    /// Returns false when an identical job is already queued/running.
    fn mark_in_flight(&self, key: &JobKey) -> bool {
        self.in_flight.lock().unwrap().insert(key.clone())
    }

    fn remove_in_flight(&self, key: &JobKey) {
        self.in_flight.lock().unwrap().remove(key);
    }
}

struct ImageQueue {
    sender: mpsc::UnboundedSender<ImageJob>,
    state: Arc<QueueState>,
}

static IMAGE_QUEUE: OnceLock<ImageQueue> = OnceLock::new();

/// Number of concurrent image workers. Each worker ≈ one busy core because
/// resize/encode operations are single-threaded.
fn worker_count() -> usize {
    if let Ok(value) = std::env::var("IMAGE_WORKERS") {
        match value.parse::<usize>() {
            Ok(n) if n > 0 => {
                info!("IMAGE_WORKERS={n} set, using {n} image workers");
                return n;
            }
            _ => error!("Invalid IMAGE_WORKERS={value:?}, falling back to auto detection"),
        }
    }
    std::thread::available_parallelism()
        .map(|n| n.get().saturating_sub(2).max(1))
        .unwrap_or(2)
}

fn init_queue() -> ImageQueue {
    let workers = worker_count();
    info!("Initializing image generation queue with {workers} workers");

    // Dedicated runtime so that image generation:
    // - works from sync contexts (askama filters) without a ambient runtime handle,
    // - never touches the main server runtime's blocking pool,
    // - is hard-capped via max_blocking_threads as defense in depth.
    // The runtime is leaked on purpose: it is a process-lifetime singleton.
    let runtime: &'static Runtime = Box::leak(Box::new(
        Builder::new_current_thread()
            .thread_name("image-gen")
            .max_blocking_threads(workers + 2)
            .build()
            .expect("Failed to build image generation runtime"),
    ));

    let (sender, receiver) = mpsc::unbounded_channel::<ImageJob>();
    let (pending, _) = watch::channel(0u64);
    let state = Arc::new(QueueState {
        in_flight: Mutex::new(HashSet::new()),
        pending,
    });
    let permits = Arc::new(Semaphore::new(workers));

    runtime.spawn(run_dispatcher(receiver, permits, state.clone()));

    // Drive the current_thread runtime forever on its own thread so the dispatcher
    // task keeps making progress regardless of the main server runtime.
    std::thread::Builder::new()
        .name("image-gen-driver".into())
        .spawn(move || runtime.block_on(std::future::pending::<()>()))
        .expect("Failed to spawn image generation driver thread");

    ImageQueue { sender, state }
}

/// Eagerly initialize the queue (optional — first `enqueue_image_job` initializes
/// lazily as well). Called from `main` so the worker count is logged at startup.
pub fn init() {
    IMAGE_QUEUE.get_or_init(init_queue);
}

/// Enqueue image generation. Fire-and-forget, sync, callable from anywhere
/// (including templates and non-tokio tests).
pub fn enqueue_image_job(job: ImageJob) {
    let queue = IMAGE_QUEUE.get_or_init(init_queue);
    queue.state.job_enqueued();
    if queue.sender.send(job).is_err() {
        error!("Image job channel closed — dispatcher is gone");
        queue.state.job_finished();
    }
}

/// Resolves once no image job is queued, running, or about to be dispatched.
/// Used by the `/export-wait` endpoint before the SSG crawl.
pub async fn wait_until_idle() {
    let queue = IMAGE_QUEUE.get_or_init(init_queue);
    let mut pending = queue.state.pending.subscribe();
    loop {
        if *pending.borrow() == 0 {
            return;
        }
        // Sender lives in the leaked queue — changed() only errors if it dropped.
        if pending.changed().await.is_err() {
            return;
        }
    }
}

async fn run_dispatcher(
    mut receiver: mpsc::UnboundedReceiver<ImageJob>,
    permits: Arc<Semaphore>,
    state: Arc<QueueState>,
) {
    while let Some(job) = receiver.recv().await {
        // Warm cache: nothing to decode or resize — cheapest possible path.
        if job.all_outputs_exist() {
            debug!(
                "Skipping image job for {:?} - all outputs already generated",
                job.disk_image_path
            );
            state.job_finished();
            continue;
        }

        // Dedup: identical job already queued or running.
        let key = job.key();
        if !state.mark_in_flight(&key) {
            debug!(
                "Skipping duplicate image job for {:?}",
                job.disk_image_path
            );
            state.job_finished();
            continue;
        }

        // Bound CPU usage: at most `permits` jobs resize/encode concurrently.
        let permit = match permits.clone().acquire_owned().await {
            Ok(permit) => permit,
            Err(err) => {
                error!("Image queue semaphore closed: {err}");
                state.remove_in_flight(&key);
                state.job_finished();
                continue;
            }
        };

        let job_state = Arc::clone(&state);
        tokio::task::spawn_blocking(move || {
            // Hold the permit for the duration of the CPU-bound work.
            let _permit = permit;
            match super::image_generator::process_job(&job) {
                Ok(_) => debug!("Processed image job for {:?}", job.disk_image_path),
                Err(err) => error!("Image job failed for {:?}: {err:#}", job.disk_image_path),
            }
            job_state.remove_in_flight(&key);
            job_state.job_finished();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_job(base: &std::path::Path) -> ImageJob {
        ImageJob {
            disk_image_path: std::path::Path::new("static/images/uploads/test.jpg").into(),
            generated_base_path: base.into(),
            resolutions: vec![(320, 200, 1.), (640, 400, 2.)],
            formats: vec![ExportFormat::Jpeg],
        }
    }

    #[test]
    fn test_job_key_ignores_density() {
        let mut job = sample_job(std::path::Path::new("/generated_images/test/img"));
        let key_with_density = job.key();
        job.resolutions = vec![(320, 200, 4.), (640, 400, 1.5)];
        assert_eq!(
            key_with_density,
            job.key(),
            "Densities must not affect job identity — output files are identical"
        );
    }

    #[test]
    fn test_job_key_differs_for_different_outputs() {
        let job = sample_job(std::path::Path::new("/generated_images/test/img"));
        let mut other = sample_job(std::path::Path::new("/generated_images/test/img_og"));
        other.resolutions = vec![(1200, 630, 1.)];
        assert_ne!(job.key(), other.key());
    }

    #[test]
    fn test_output_paths_cover_all_variants() {
        let mut job = sample_job(std::path::Path::new("/generated_images/test/img"));
        job.formats = vec![ExportFormat::Jpeg, ExportFormat::Png];
        let outputs = job.output_paths();
        assert_eq!(outputs.len(), 4, "2 formats x 2 resolutions");
        assert!(outputs
            .iter()
            .all(|path| path.to_str().unwrap().starts_with("./generated_images/test/")));
        assert!(outputs.iter().any(|path| path
            .to_str()
            .unwrap()
            .ends_with("img_320x200.jpg")));
        assert!(outputs.iter().any(|path| path
            .to_str()
            .unwrap()
            .ends_with("img_640x400.png")));
    }

    #[test]
    fn test_all_outputs_exist_requires_every_file() {
        // Anchor under the crate so that get_save_path's CWD-relative mapping
        // (strips leading "/", prefixes "./") resolves to the same directory.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("target/image-tests/jobs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("img");
        let job = sample_job(&base);

        assert!(!job.all_outputs_exist());

        // Create only one of the two expected outputs.
        let first = get_save_path(&base, 320, 200, ExportFormat::Jpeg);
        std::fs::create_dir_all(first.parent().unwrap()).unwrap();
        std::fs::write(&first, b"x").unwrap();
        assert!(!job.all_outputs_exist(), "Partial cache is not warm");

        let second = get_save_path(&base, 640, 400, ExportFormat::Jpeg);
        std::fs::write(&second, b"x").unwrap();
        assert!(job.all_outputs_exist());

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
