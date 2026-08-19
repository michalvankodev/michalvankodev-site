# Image Generation Refactor

> Status: **done** — implemented & verified (cold generation, warm-cache skip, dedup,
> bounded CPU, full SSG export with prewarm)
> Goal: Replace rayon-based image generation with a tokio-based dispatcher, bounded worker
> pool, and a restructured generation algorithm that never saturates the CPU.

## Problem statement

`src/picture_generator/` currently uses rayon for parallelization:

1. `rayon::spawn` (fire-and-forget) in `picture_markup_generator.rs` + `image_src_generator.rs`
2. `formats.par_iter() × resolutions.par_iter()` in `image_generator.rs`

Issues:

- **100% CPU** — rayon's global pool uses all cores; the machine becomes unusable during
  generation (dev server, SSG export).
- **No deduplication** — the same image rendered on multiple pages spawns duplicate jobs
  that race writing the same output files.
- **Warm cache still decodes + resizes** — the `save_path.exists()` check happens *after*
  `resize_to_fill`, so fully-generated sites still pay full CPU cost on every render.
- **Resize runs per format×resolution** — the same resolution is resized once per format.
- **Non-atomic writes** — `wget` can crawl a half-written file into `dist/`.
- **One `exiftool` subprocess per output file.**
- `todo!()` on Avif/Svg panics inside detached tasks.

## Design

```
askama filter / template (sync)
   │  enqueue_image_job()   — sync send on unbounded mpsc (works without a runtime handle)
   ▼
Dispatcher actor (single task on a dedicated leaked current_thread tokio runtime,
driven by a "image-gen-driver" thread)
   ├─ warm-cache check: if ALL target files exist → skip (no decode!)
   ├─ dedup: skip if identical job already queued/running
   ├─ acquire semaphore permit (permits = worker cap)
   ▼
spawn_blocking worker (≤ N concurrent, N = worker cap)
   decode once → resize once per resolution → encode per format
   → write `<file>.tmp` + atomic rename → ONE batched exiftool call per job
```

### Worker cap (CPU dial)

- Each worker ≈ one busy core (image resize/encode is single-threaded per op).
- Default: `(available_parallelism() − 2).clamp(1, ..)` → 6 on the 8-core dev machine.
- Override: `IMAGE_WORKERS=<n>` env var.
- The dedicated runtime also caps `max_blocking_threads` as defense in depth.

## Out of scope (deliberate non-goals)

- **EXIF orientation fix** (rotating pixels at decode, dropping the width/height swap
  workaround) — behavior change, to be done as its own commit later.
- Avif/Svg encoding — remains not enabled (`get_export_formats` never returns them);
  unreachable arms now return errors instead of `todo!()`.

## Task list

- [x] Analyze current code and write this spec
- [x] `src/picture_generator/image_jobs.rs` — new: `ImageJob`, job keys, global queue
      (lazy `OnceLock` init), dispatcher actor, semaphore, dedup, warm-cache skip,
      `wait_until_idle()` barrier
- [x] `src/picture_generator/image_generator.rs` — rewrite worker fn:
      `process_job` (decode once, resize once per resolution, formats inner loop),
      `get_save_path` helper, atomic tmp+rename writes, explicit BufWriter flush,
      batched `copy_exif`, no rayon, no `todo!()`
- [x] `src/picture_generator/export_format.rs` — add `Eq`/`Hash` derives (job keys)
- [x] `src/picture_generator/picture_markup_generator.rs` — replace `rayon::spawn`
      with `enqueue_image_job`, drop Arc juggling
- [x] `src/picture_generator/image_src_generator.rs` — same
- [x] `src/picture_generator/mod.rs` — export new module
- [x] `src/pages/export_wait.rs` + `src/router.rs` — `GET /export-wait` long-polls
      `wait_until_idle()`
- [x] `src/main.rs` — eager queue init at startup (logs worker count)
- [x] `justfile` — `ssg_prewarm` recipe: crawl (no requisites) → curl `/export-wait`
      → real crawl; wired into `ssg`
- [x] `Cargo.toml` — remove `rayon`
- [x] Tests: job-key dedup semantics, `output_paths`, `process_job` generation +
      warm-cache skip + atomic artifacts (temp dirs), existing tests still pass
- [x] Verify: `cargo build`, `cargo test`, manual smoke run of server + export flow

## Verification results

- `cargo build` — 0 errors, 0 warnings; `cargo test` — 14 passed, 0 failed
  (new: job-key dedup semantics, `output_paths`, warm-cache detection, `process_job`
  generation + partial-cache fill + valid output images, no `.tmp` leftovers)
- Cold start (dev server, `PORT=3099`): page render returns immediately (fire-and-forget),
  queue log shows `Initializing image generation queue with 6 workers`, all variants
  generated after `/export-wait` drain, 0 `.tmp` leftovers
- Warm re-render: dispatcher logs `Skipping image job ... all outputs already generated`
  (no decode/resize at all)
- Double render of the same page: `Skipping duplicate image job` logged exactly once
- `just export` (release build): prewarm crawl = pages only (388 files, 0 errors);
  real crawl = **388 files, 123 MB, 0 errors** — every generated image existed and was
  complete; `dist/` = 403 files incl. 303 valid images

## Pre-existing breakage fixed along the way (required to build/verify at all)

The tree did not compile before this refactor (untracked `Cargo.lock` had drifted):

1. **askama pinned back to 0.14** (`Cargo.toml`, askama_escape to 0.14): renovate bumped
   askama to 0.15 (commit 2cbe8b8, 2025-12-22) but the templates were never migrated —
   0.15 requires `{% extends %}` as the very first tag and `{% call %}` to be closed
   with `{% endcall %}`. The site's templates (and filter signatures in
   `src/filters/markdown.rs`) are written for 0.14. **Follow-up**: done — upgraded to
   askama 0.16 in a separate commit (`feat(deps): upgrade askama from 0.14 to 0.16`).
2. **gray_matter 0.3 API** (`src/post_utils/post_parser.rs`): `parse_with_struct` →
   `parse::<T>()` + `.data` is now `Option<T>`.
3. **Template order** (`templates/index.html`, `portfolio.html`, `post_list.html`):
   moved `{% extends %}` above `{% import %}` (also required by 0.15, harmless on 0.14).
4. **Recommendation**: commit `Cargo.lock` so CI/dev builds are reproducible and
   dependency drift like this is caught in review.
