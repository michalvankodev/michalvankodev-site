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

/// Static raster fallback for og:image/twitter:image. Card crawlers (X,
/// Facebook, LinkedIn, Mastodon) cannot render SVG, so SVG thumbnails and
/// missing thumbnails get this 1200x630 brand image instead.
pub const OG_DEFAULT_IMAGE: &str = "/images/og-default.png";

/// Used directly in templates: best raster og:image src for a post thumbnail.
/// Raster thumbnails get their generated 1200x630 `_og` variant; SVG sources
/// and missing thumbnails get the static default card image.
pub fn og_image_src(thumbnail: Option<&str>) -> String {
    match thumbnail {
        Some(src) if !src.to_lowercase().ends_with(".svg") => {
            generate_image_with_src(src, 1200, 630, "_og")
                .unwrap_or_else(|_| OG_DEFAULT_IMAGE.to_string())
        }
        _ => OG_DEFAULT_IMAGE.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_thumbnail_gets_default_card_image() {
        assert_eq!(
            og_image_src(Some("/images/uploads/pi-logo.svg")),
            OG_DEFAULT_IMAGE
        );
    }

    #[test]
    fn uppercase_svg_extension_is_also_caught() {
        assert_eq!(
            og_image_src(Some("/images/uploads/LOGO.SVG")),
            OG_DEFAULT_IMAGE
        );
    }

    #[test]
    fn missing_thumbnail_gets_default_card_image() {
        assert_eq!(og_image_src(None), OG_DEFAULT_IMAGE);
    }

    #[test]
    fn raster_thumbnail_gets_generated_og_variant() {
        let src = og_image_src(Some("/images/uploads/2020-03-23_20-24-06_393.jpg"));
        assert!(
            src.contains("_og"),
            "raster thumbnails get the generated _og variant, got: {src}"
        );
    }
}
