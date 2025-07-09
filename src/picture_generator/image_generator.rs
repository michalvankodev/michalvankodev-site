use std::{fs::create_dir_all, path::Path};

use image::{imageops::FilterType, DynamicImage};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::{debug, error};

use super::export_format::ExportFormat;

pub fn generate_images(
    image: &DynamicImage,
    disk_image_path: &Path,
    path_to_generated: &Path,
    resolutions: &[(u32, u32, f32)],
    formats: &[ExportFormat],
) -> Result<(), anyhow::Error> {
    formats.par_iter().for_each(|format| {
        resolutions.par_iter().for_each(|resolution| {
            let (width, height, _) = *resolution;
            let resized = image.resize_to_fill(width, height, FilterType::Triangle);
            let file_name = path_to_generated.file_name().unwrap().to_str().unwrap();
            let save_path = Path::new("./")
                .join(path_to_generated.strip_prefix("/").unwrap())
                .with_file_name(format!("{file_name}_{width}x{height}"))
                .with_extension(format.get_extension());

            if save_path.exists() {
                debug!("Skip generating {save_path:?} - Already exists");
                return;
            }

            let parent_dir = save_path.parent().unwrap();
            if !parent_dir.exists() {
                create_dir_all(parent_dir).unwrap();
            }

            let result = resized.save_with_format(&save_path, format.get_image_format());
            match result {
                Err(err) => {
                    error!("Failed to generate {:?} - {:?}", &save_path, err);
                }
                Ok(_) => {
                    debug!("Generated image {:?}", &save_path);
                    let _ = copy_exif(disk_image_path, &save_path);
                }
            }
        });
    });

    Ok(())
}

pub fn copy_exif(orig_path: &Path, save_path: &Path) -> Result<(), anyhow::Error> {
    let status = std::process::Command::new("exiftool")
        .args([
            "-TagsFromFile",
            orig_path.to_str().expect("Orig path should exist"),
            "-exif:all",
            "-overwrite_original",
            save_path.to_str().expect("Save path of image should exist"),
        ])
        .status()?;

    if status.success() {
        debug!("EXIF copied successfully.");
    } else {
        error!("Failed to copy EXIF.");
    }
    Ok(())
}
