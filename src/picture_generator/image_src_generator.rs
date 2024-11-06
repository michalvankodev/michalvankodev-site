use std::{path::Path, sync::Arc};

use anyhow::Context;
use image::ImageReader;

use super::{
    image_generator::generate_images,
    picture_markup_generator::{get_export_formats, get_generated_file_name, get_image_path},
};

pub fn generate_image_with_src(
    orig_img_path: &str,
    width: u32,
    height: u32,
    suffix: &str,
) -> Result<String, anyhow::Error> {
    let path_to_generated = get_generated_file_name(orig_img_path);
    let file_stem = path_to_generated.file_stem().unwrap().to_str().unwrap();
    let path_to_generated = path_to_generated.with_file_name(format!("{file_stem}{suffix}"));

    let disk_img_path =
        Path::new("static/").join(orig_img_path.strip_prefix("/").unwrap_or(orig_img_path));
    let resolutions = [(width, height, 1.)];

    let exported_formats = get_export_formats(orig_img_path);

    if exported_formats.is_empty() {
        return Ok(orig_img_path.to_string());
    }

    let exported_format = *exported_formats.first().unwrap();

    let path_to_generated_arc = Arc::new(path_to_generated);
    let path_to_generated_clone = Arc::clone(&path_to_generated_arc);

    rayon::spawn(move || {
        let orig_img = ImageReader::open(&disk_img_path)
            .with_context(|| format!("Failed to read instrs from {:?}", &disk_img_path))
            .unwrap()
            .decode()
            .unwrap();
        let path_to_generated = path_to_generated_clone.as_ref();

        let result = generate_images(
            &orig_img,
            path_to_generated,
            &resolutions,
            &[exported_format],
        )
        .with_context(|| "Failed to generate images".to_string());
        if let Err(e) = result {
            tracing::error!("Error: {}", e);
        }
    });

    let path_to_generated = Arc::clone(&path_to_generated_arc);

    let image_path = get_image_path(
        &path_to_generated,
        resolutions.first().expect("Should this error ever happen?"),
        &exported_format,
    );

    Ok(image_path)
}
