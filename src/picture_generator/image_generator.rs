use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use image::{
    codecs::{jpeg::JpegEncoder, png::PngEncoder},
    imageops::FilterType,
    DynamicImage,
};
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

            let encode = {
                let buffered_file_write = &mut BufWriter::new(File::create(&save_path).unwrap()); // always seekable
                match format {
                    ExportFormat::Jpeg => {
                        let encoder = JpegEncoder::new_with_quality(buffered_file_write, 87);
                        resized.write_with_encoder(encoder)
                    }
                    ExportFormat::Avif => todo!(),
                    ExportFormat::Svg => todo!(),
                    ExportFormat::Png => {
                        let encoder = PngEncoder::new_with_quality(
                            buffered_file_write,
                            image::codecs::png::CompressionType::Best,
                            image::codecs::png::FilterType::Adaptive,
                        );
                        resized.write_with_encoder(encoder)
                    }
                }
            };
            match encode {
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
