use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::{Path, PathBuf},
};
use image::{
    codecs::{jpeg::JpegEncoder, png::PngEncoder},
    imageops::FilterType,
    DynamicImage, ImageError,
};
use std::io::Write as _;
use tracing::{debug, error, warn};

use super::{export_format::ExportFormat, image_jobs::ImageJob};

/// Compute the on-disk path for one generated variant of an image.
///
/// `generated_base_path` looks like `/generated_images/images/uploads/img_name`
/// (possibly with a suffix like `_og` before the dimensions); the result is the
/// relative `./generated_images/.../img_name_{width}x{height}.{ext}` path.
pub fn get_save_path(
    generated_base_path: &Path,
    width: u32,
    height: u32,
    format: ExportFormat,
) -> PathBuf {
    let file_name = generated_base_path.file_name().unwrap().to_str().unwrap();
    Path::new("./")
        .join(generated_base_path.strip_prefix("/").unwrap())
        .with_file_name(format!("{file_name}_{width}x{height}"))
        .with_extension(format.get_extension())
}

/// Generate all variants of one image job. Runs on a blocking thread, bounded by
/// the image queue's semaphore (see `image_jobs`).
///
/// Algorithm:
/// 1. decode the original once,
/// 2. resize once per resolution (not per format × resolution),
/// 3. encode each format from the already-resized image,
/// 4. write to a temporary file and atomically rename it into place — readers
///    (e.g. the SSG crawler) never observe half-written files,
/// 5. copy EXIF data once for the whole batch with a single `exiftool` call.
pub fn process_job(job: &ImageJob) -> Result<(), anyhow::Error> {
    let ImageJob {
        disk_image_path,
        generated_base_path,
        resolutions,
        formats,
    } = job;

    if resolutions.is_empty() || formats.is_empty() {
        debug!("Nothing to generate for {disk_image_path:?}");
        return Ok(());
    }

    let orig_img = image::ImageReader::open(disk_image_path)
        .map_err(|err| anyhow::anyhow!("Failed to open {disk_image_path:?}: {err}"))?
        .decode()
        .map_err(|err| anyhow::anyhow!("Failed to decode {disk_image_path:?}: {err}"))?;

    let mut outputs: Vec<PathBuf> = Vec::new();

    for (width, height, _density) in resolutions {
        // One resize per resolution, shared by all formats.
        let resized = orig_img.resize_to_fill(*width, *height, FilterType::Triangle);

        for format in formats {
            let save_path = get_save_path(generated_base_path, *width, *height, *format);
            if save_path.exists() {
                debug!("Skip generating {save_path:?} - already exists");
                outputs.push(save_path);
                continue;
            }

            if let Some(parent_dir) = save_path.parent() {
                if !parent_dir.exists() {
                    create_dir_all(parent_dir)?;
                }
            }

            let tmp_path = with_tmp_extension(&save_path);
            if let Err(err) = encode_image(&resized, *format, &tmp_path) {
                // Best effort cleanup so a failed encode does not leave a partial file.
                let _ = std::fs::remove_file(&tmp_path);
                error!("Failed to generate {save_path:?}: {err}");
                continue;
            }

            // Atomic publish: readers either see the complete file or nothing.
            std::fs::rename(&tmp_path, &save_path)?;
            debug!("Generated image {save_path:?}");
            outputs.push(save_path);
        }
    }

    if !outputs.is_empty() {
        // EXIF copying is best-effort: exiftool may not be installed (e.g. CI),
        // and metadata must never fail an otherwise successful generation.
        if let Err(err) = copy_exif(disk_image_path, &outputs) {
            warn!("exiftool failed for {disk_image_path:?}: {err}");
        }
    }

    Ok(())
}

/// `foo_320x200.jpg` → `foo_320x200.jpg.tmp`
fn with_tmp_extension(path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(".tmp");
    PathBuf::from(tmp)
}

fn encode_image(
    image: &DynamicImage,
    format: ExportFormat,
    path: &Path,
) -> Result<(), ImageError> {
    let mut writer = BufWriter::new(File::create(path).map_err(ImageError::IoError)?);
    let result = match format {
        ExportFormat::Jpeg => {
            image.write_with_encoder(JpegEncoder::new_with_quality(&mut writer, 87))
        }
        ExportFormat::Png => {
            image.write_with_encoder(PngEncoder::new_with_quality(
                &mut writer,
                image::codecs::png::CompressionType::Best,
                image::codecs::png::FilterType::Adaptive,
            ))
        }
        // Not selectable via `get_export_formats` yet — do not panic in worker
        // threads, just refuse.
        ExportFormat::Avif | ExportFormat::Svg => {
            warn!("Skipping unsupported export format {format:?}");
            return Ok(());
        }
    };
    // Make sure buffered bytes hit the file before the atomic rename.
    writer.flush().map_err(ImageError::IoError)?;
    result
}

/// Copy EXIF metadata from the original to all generated variants with a single
/// `exiftool` invocation (one process per job instead of one per file).
pub fn copy_exif(orig_path: &Path, outputs: &[PathBuf]) -> Result<(), anyhow::Error> {
    if outputs.is_empty() {
        return Ok(());
    }

    let orig = orig_path.to_str().expect("Orig path should exist");
    let mut command = std::process::Command::new("exiftool");
    command
        .arg("-TagsFromFile")
        .arg(orig)
        .arg("-exif:all")
        .arg("-overwrite_original");
    for output in outputs {
        command.arg(output.to_str().expect("Save path of image should exist"));
    }

    let status = command.status()?;

    if status.success() {
        debug!("EXIF copied successfully to {} file(s).", outputs.len());
    } else {
        error!("Failed to copy EXIF to {} file(s).", outputs.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::picture_generator::image_jobs::ImageJob;
    use image::{DynamicImage, RgbImage};

    fn temp_base(label: &str) -> PathBuf {
        // get_save_path() strips a leading "/" and prefixes "./" — i.e. it maps
        // absolute paths onto CWD-relative ones. During tests the CWD is the crate
        // root, so anchoring under CARGO_MANIFEST_DIR makes both views identical.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("target/image-tests/{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_test_source(dir: &Path) -> PathBuf {
        let source = dir.join("source.jpg");
        let img = DynamicImage::ImageRgb8(RgbImage::new(1000, 600));
        img.save_with_format(&source, image::ImageFormat::Jpeg).unwrap();
        source
    }

    #[test]
    fn test_get_save_path() {
        let base = Path::new("/generated_images/images/uploads/img_name");
        assert_eq!(
            get_save_path(base, 320, 200, ExportFormat::Jpeg),
            PathBuf::from("./generated_images/images/uploads/img_name_320x200.jpg")
        );
        assert_eq!(
            get_save_path(base, 640, 400, ExportFormat::Png),
            PathBuf::from("./generated_images/images/uploads/img_name_640x400.png")
        );
    }

    #[test]
    fn test_process_job_generates_all_variants_and_skips_warm_cache() {
        let base_dir = temp_base("full");
        let source = write_test_source(&base_dir);
        let generated_base = base_dir.join("gen/img");

        let mut job = ImageJob {
            disk_image_path: source,
            generated_base_path: generated_base.clone(),
            resolutions: vec![(100, 60, 1.), (200, 120, 2.)],
            formats: vec![ExportFormat::Jpeg],
        };
        process_job(&job).expect("job should succeed");

        for (width, height, _) in &job.resolutions {
            let path = get_save_path(&generated_base, *width, *height, ExportFormat::Jpeg);
            assert!(path.exists(), "missing generated file {path:?}");
            // No leftover temp files after successful runs.
            assert!(!with_tmp_extension(&path).exists());
        }

        // Warm cache: outputs already exist → job re-runs must not fail and must not
        // clobber them. The queue-level `all_outputs_exist` check avoids decode
        // entirely; this guards the per-file skip path for partial caches.
        job.resolutions.push((999, 1, 3.));
        process_job(&job).expect("warm job should succeed");
        assert!(
            get_save_path(&generated_base, 999, 1, ExportFormat::Jpeg).exists(),
            "partial cache fills only the missing variants"
        );

        std::fs::remove_dir_all(&base_dir).unwrap();
    }

    #[test]
    fn test_generated_files_are_valid_images() {
        let base_dir = temp_base("valid");
        let source = write_test_source(&base_dir);
        let generated_base = base_dir.join("gen/img");

        let job = ImageJob {
            disk_image_path: source,
            generated_base_path: generated_base.clone(),
            resolutions: vec![(64, 64, 1.)],
            formats: vec![ExportFormat::Jpeg, ExportFormat::Png],
        };
        process_job(&job).unwrap();

        // 64x64 crop from 1000x600 (resize_to_fill crops to aspect ratio)
        let jpg = image::ImageReader::open(get_save_path(&generated_base, 64, 64, ExportFormat::Jpeg))
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        assert_eq!((64, 64), (jpg.width(), jpg.height()));

        let png = image::ImageReader::open(get_save_path(&generated_base, 64, 64, ExportFormat::Png))
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        assert_eq!((64, 64), (png.width(), png.height()));

        std::fs::remove_dir_all(&base_dir).unwrap();
    }
}
