use std::path::Path;

use super::{
    image_jobs::{enqueue_image_job, ImageJob},
    picture_markup_generator::{get_export_formats, get_generated_file_name, get_image_path},
};

/// Used directly in templates
pub fn generate_image_with_src(
    orig_img_path: &str,
    width: u32,
    height: u32,
    suffix: &str,
) -> Result<String, anyhow::Error> {
    let orig_path = Path::new(orig_img_path);
    let path_to_generated = get_generated_file_name(orig_path);
    let file_stem = path_to_generated.file_stem().unwrap().to_str().unwrap();
    let path_to_generated = path_to_generated.with_file_name(format!("{file_stem}{suffix}"));

    let disk_img_path =
        Path::new("static/").join(orig_img_path.strip_prefix("/").unwrap_or(orig_img_path));
    let resolutions = vec![(width, height, 1.)];

    let exported_formats = get_export_formats(orig_path);

    if exported_formats.is_empty() {
        return Ok(orig_img_path.to_string());
    }

    let exported_format = *exported_formats.first().unwrap();

    // Fire-and-forget: the queue deduplicates jobs, skips fully generated
    // images, and bounds CPU usage (see `image_jobs`).
    enqueue_image_job(ImageJob {
        disk_image_path: disk_img_path,
        generated_base_path: path_to_generated.clone(),
        resolutions,
        formats: exported_formats,
    });

    let image_path = get_image_path(&path_to_generated, &(width, height, 1.), &exported_format);

    Ok(image_path)
}
