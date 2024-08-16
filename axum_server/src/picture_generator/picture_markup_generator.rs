use std::path::{Path, PathBuf};

pub const PIXEL_DENSITIES: [f32; 5] = [1., 1.5, 2., 3., 4.];

#[derive(Debug, PartialEq)]
pub enum ExportFormat {
    JPG,
    AVIF,
    SVG,
    PNG,
}

pub fn generate_picture_markup(orig_img_path: &str, width: u32, height: u32) -> String {
    let exported_formats = get_export_formats(orig_img_path);
    let path_to_generated = get_generated_file_name(orig_img_path);
    let resolutions = vec![
        (300, 200, 1.),
        (450, 300, 1.5),
        (600, 400, 2.),
        (900, 600, 3.),
        (1200, 800, 4.),
    ];

    for export_format in exported_formats {

        // TODO get original img resolution and determine how many exports are going to be needed
        // let orig_img_resolution =
        // let resolutions = get_resolutions(width, height);
    }
    let result = format!("<picture></picture>");
    result
}

fn get_generated_file_name(orig_img_path: &str) -> PathBuf {
    let path = Path::new(&orig_img_path);
    let parent = path
        .parent()
        .expect("There should be a parent route to an image")
        .strip_prefix(".")
        .unwrap();
    let file_name = path
        .file_stem()
        .expect("There should be a name for every img");
    let result = Path::new("./generated_images/")
        .join(parent)
        .join(file_name);
    result
}

#[test]
fn test_get_generated_paths() {
    let orig_img_path = "./images/uploads/img_name.jpg";
    assert_eq!(
        get_generated_file_name(orig_img_path)
            .to_str()
            .unwrap_or(""),
        "./generated_images/images/uploads/img_name"
    );
}

// TODO get original img resolution and determine how many exports are going to be needed
fn get_resolutions(orig_width: i32, orig_height: i32, width: i32, height: i32) -> Vec<(i32, i32)> {
    todo!("get original img resolution and determine how many exports are going to be needed")
}

fn generate_srcset(new_path: &str, width: u32, height: u32) -> &str {
    todo!("generate srcset")
}

fn get_export_formats(orig_img_path: &str) -> Vec<ExportFormat> {
    let path = Path::new(&orig_img_path)
        .extension()
        .and_then(|ext| ext.to_str());

    match path {
        Some("jpg" | "jpeg") => vec![ExportFormat::AVIF, ExportFormat::JPG],
        Some("png") => vec![ExportFormat::AVIF, ExportFormat::PNG],
        Some(_) | None => vec![],
    }
}

#[test]
fn test_get_export_formats() {
    assert_eq!(
        get_export_formats("./images/uploads/img_name.jpg"),
        vec![ExportFormat::AVIF, ExportFormat::JPG]
    )
}
#[test]
fn test_generate_srcset() {
    let width = 400;
    let height = 200;
    let orig_img_path = "./images/uploads/img_name.jpg";
    let result = "./generated_images/images/uploads/img_name_400x200.avif 1x, ./generated_images/images/uploads/img_name_500x300.avif 1.5x, ./generated_images/images/uploads/img_name_800x400.avif 2x, ./generated_images/images/uploads/img_name_1200x600.avif 3x, ./generated_images/images/uploads/img_name_1600x800.avif 4x";
    assert_eq!(generate_srcset(orig_img_path, width, height), result)
}

#[test]
fn test_generate_picture_markup() {
    let width = 300;
    let height = 200;
    let orig_img_path = "./images/uploads/img_name.jpg";
    let result = r#"""
        <picture>
            <source
                srcset="./generated_images/images/uploads/img_name_300x200.avif 1x, ./generated_images/images/uploads/img_name_450x300.avif 1.5x, ./generated_images/images/uploads/img_name_600x400.avif 2x, ./generated_images/images/uploads/img_name_900x600.avif 3x, ./generated_images/images/uploads/img_name_1200x800.avif 4x"
                type="image/avif"
            > 
            <source
                srcset="./generated_images/images/uploads/img_name_300x200.jpg 1x, ./generated_images/images/uploads/img_name_450x300.jpg 1.5x, ./generated_images/images/uploads/img_name_600x400.jpg 2x, ./generated_images/images/uploads/img_name_900x600.jpg 3x, ./generated_images/images/uploads/img_name_1200x800.jpg 4x"
                type="image/jpeg"
            >
            <img
                src="./generated_images/images/uploads/img_name_300x200.jpg"
                width="300"
                height="200"
            >
        </picture>
    """#;
    assert_eq!(
        generate_picture_markup(orig_img_path, width, height),
        result
    );
}
