use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use image::{image_dimensions, ImageDecoder, ImageReader};
use indoc::formatdoc;

use super::{
    export_format::ExportFormat, image_generator::generate_images,
    resolutions::get_max_resolution_with_crop,
};

pub const PIXEL_DENSITIES: [f32; 5] = [1., 1.5, 2., 3., 4.];

/// Used by markdown generator
pub fn generate_picture_markup(
    orig_img_path: &str,
    width: u32,
    height: u32,
    alt_text: &str,
    class_name: Option<&str>,
) -> Result<String, anyhow::Error> {
    let orig_path = Path::new(orig_img_path);
    let exported_formats = get_export_formats(orig_path);
    let class_attr = if let Some(class) = class_name {
        format!(r#"class="{class}""#)
    } else {
        "".to_string()
    };

    // Here the resolution is already correct
    if exported_formats.is_empty() {
        return Ok(formatdoc!(
            r#"<img
            src="{orig_img_path}"
            width="{width}"
            height="{height}"
            {class_attr}
            alt="{alt_text}"
        >"#
        ));
    }
    let path_to_generated = get_generated_file_name(orig_path);

    let disk_img_path =
        Path::new("static/").join(orig_img_path.strip_prefix("/").unwrap_or(orig_img_path));

    // Here we have a problem. The resolution is swapped but we want to generate images with original dimensions which are not here.
    let orig_img_dimensions = image_dimensions(&disk_img_path)?;
    let resolutions = get_resolutions(
        orig_img_dimensions,
        width,
        height,
        should_swap_dimensions(&disk_img_path),
    );
    let path_to_generated_arc = Arc::new(path_to_generated);
    let path_to_generated_clone = Arc::clone(&path_to_generated_arc);
    let resolutions_arc = Arc::new(resolutions);
    let resolutions_clone = Arc::clone(&resolutions_arc);
    let exported_formats_arc = Arc::new(exported_formats);
    let exported_formats_clone = Arc::clone(&exported_formats_arc);

    rayon::spawn(move || {
        let orig_img = ImageReader::open(&disk_img_path)
            .with_context(|| format!("Failed to read instrs from {:?}", &disk_img_path))
            .unwrap()
            .decode()
            .unwrap();

        let result = generate_images(
            &orig_img,
            &disk_img_path,
            &path_to_generated_clone,
            &resolutions_clone,
            &exported_formats_clone,
        )
        .with_context(|| "Failed to generate images".to_string());
        if let Err(e) = result {
            tracing::error!("Error: {}", e);
        }
    });

    let exported_formats = Arc::clone(&exported_formats_arc);
    let path_to_generated = Arc::clone(&path_to_generated_arc);
    let resolutions = Arc::clone(&resolutions_arc);

    let source_tags = exported_formats
        .iter()
        .map(|format| {
            let srcset = generate_srcset(&path_to_generated, format, &resolutions);
            let format_type = format.get_type();
            formatdoc!(
                r#"<source
                srcset="{srcset}"
                type="{format_type}"
            >"#
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let image_path = get_image_path(
        &path_to_generated,
        resolutions.first().expect("Should this error ever happen?"),
        exported_formats.last().expect("Can this one ever happen?"),
    );
    let image_tag = formatdoc!(
        r#"<img
            src="{image_path}"
            width="{width}"
            height="{height}"
            alt="{alt_text}"
            {class_attr}
        >"#
    );

    let result = formatdoc!(
        r#"<picture>
            {source_tags}
            {image_tag}
        </picture>"#
    );

    Ok(result)
}

pub fn get_image_path(path: &Path, resolution: &(u32, u32, f32), format: &ExportFormat) -> String {
    let path_name = path.to_str().expect("Image has to have a valid path");
    let (width, height, _) = resolution;
    let extension = format.get_extension();

    format!("{path_name}_{width}x{height}.{extension}")
}

/// Take original resolution of photo and
/// exif data is not taken into consideration therefore we don't need to do anything here regarding to swapping width-height
fn get_resolutions(
    (orig_width, orig_height): (u32, u32),
    width: u32,
    height: u32,
    swap_dimensions: bool,
) -> Vec<(u32, u32, f32)> {
    let (width, height) = if swap_dimensions {
        (height, width)
    } else {
        (width, height)
    };
    let mut resolutions: Vec<(u32, u32, f32)> = vec![];
    for pixel_density in PIXEL_DENSITIES {
        let (density_width, density_height) = (
            (pixel_density * width as f32) as u32,
            (pixel_density * height as f32) as u32,
        );

        // The equal sign `=` was added just to prevent next occurence of the loop
        // In case of the `orig_width` and `orig_height` being the same as `width` and `height`
        // See test case #1
        if density_width >= orig_width || density_height >= orig_height {
            let (max_width, max_height) =
                get_max_resolution_with_crop((orig_width, orig_height), width, height);
            resolutions.push((max_width, max_height, pixel_density));
            break;
        }
        resolutions.push((density_width, density_height, pixel_density));
    }
    resolutions
}

#[test]
fn test_get_resolutions() {
    assert_eq!(
        get_resolutions((320, 200), 320, 200, false),
        vec![(320, 200, 1.)],
        "Only original size fits"
    );
    assert_eq!(
        get_resolutions((500, 400), 320, 200, false),
        vec![(320, 200, 1.), (480, 300, 1.5), (500, 312, 2.)],
        "Should only create sizes that fits and fill the max possible for the last one - width"
    );
    assert_eq!(
        get_resolutions((400, 600), 300, 200, false),
        vec![(300, 200, 1.), (400, 266, 1.5)],
        "Should only create sizes that fits and fill the max possible for the last one - height"
    );
    assert_eq!(
        get_resolutions((1200, 900), 320, 200, false),
        vec![
            (320, 200, 1.),
            (480, 300, 1.5),
            (640, 400, 2.),
            (960, 600, 3.),
            (1200, 750, 4.)
        ],
        "Should create all possible sizes, with the last one maxed"
    );
    // TODO add test for swapping
}

fn strip_prefixes(path: &Path) -> &Path {
    // Loop to remove all leading "../" components
    let mut parent_path = path
        .parent()
        .expect("there should be a parent route to an image");
    while let Ok(stripped) = parent_path
        .strip_prefix("..")
        .or(parent_path.strip_prefix("."))
    {
        parent_path = stripped;
    }
    parent_path
}

pub fn get_generated_file_name(orig_img_path: &Path) -> PathBuf {
    let parent = strip_prefixes(orig_img_path);
    let file_name = orig_img_path
        .file_stem()
        .expect("There should be a name for every img");
    let result = Path::new("/generated_images/")
        .join(
            parent
                .strip_prefix("/")
                .unwrap_or(Path::new("/generated_images/")),
        )
        .join(file_name);
    result
}

#[test]
fn test_get_generated_paths() {
    let orig_img_path = Path::new("/images/uploads/img_name.jpg");
    assert_eq!(
        get_generated_file_name(orig_img_path)
            .to_str()
            .unwrap_or(""),
        "/generated_images/images/uploads/img_name"
    );
}

fn generate_srcset(path: &Path, format: &ExportFormat, resolutions: &[(u32, u32, f32)]) -> String {
    resolutions
        .iter()
        .map(|resolution| {
            let extension = format.get_extension();
            let (width, height, density) = resolution;
            let path_name = path.to_str().expect("Path to an image has to be valid");
            let formatted_density = if density.fract() == 0.0 {
                format!("{density}") // Convert to integer if there is no decimal part
            } else {
                format!("{density:.1}") // Format to 1 decimal place if there is a fractional part
            };
            format!("{path_name}_{width}x{height}.{extension} {formatted_density}x")
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn get_export_formats(orig_img_path: &Path) -> Vec<ExportFormat> {
    let path = orig_img_path.extension().and_then(|ext| ext.to_str());

    match path {
        // THINK: Do we want to enable avif? It's very expensive to encode
        // Some("jpg" | "jpeg") => vec![ExportFormat::Avif, ExportFormat::Jpeg],
        // Some("png") => vec![ExportFormat::Avif, ExportFormat::Png],
        Some("jpg" | "jpeg") => vec![ExportFormat::Jpeg],
        Some("png") => vec![ExportFormat::Png],
        Some(_) | None => vec![],
    }
}

pub fn should_swap_dimensions(img_path: &Path) -> bool {
    let orientation = ImageReader::open(img_path)
        .unwrap()
        .into_decoder()
        .unwrap()
        .orientation()
        .unwrap();

    matches!(
        orientation,
        image::metadata::Orientation::Rotate90
            | image::metadata::Orientation::Rotate270
            | image::metadata::Orientation::Rotate90FlipH
            | image::metadata::Orientation::Rotate270FlipH
    )
}

#[test]
fn test_get_export_formats() {
    assert_eq!(
        get_export_formats(Path::new("/images/uploads/img_name.jpg")),
        // vec![ExportFormat::Avif, ExportFormat::Jpeg]
        vec![ExportFormat::Jpeg]
    )
}
#[test]
fn test_generate_srcset() {
    let orig_img_path =
        <PathBuf as std::str::FromStr>::from_str("/generated_images/images/uploads/img_name")
            .unwrap();
    let export_format = ExportFormat::Avif;
    let resolutions = vec![
        (320, 200, 1.),
        (480, 300, 1.5),
        (640, 400, 2.),
        (960, 600, 3.),
        (1200, 750, 4.),
    ];
    let result = "/generated_images/images/uploads/img_name_320x200.avif 1x, /generated_images/images/uploads/img_name_480x300.avif 1.5x, /generated_images/images/uploads/img_name_640x400.avif 2x, /generated_images/images/uploads/img_name_960x600.avif 3x, /generated_images/images/uploads/img_name_1200x750.avif 4x";
    assert_eq!(
        generate_srcset(&orig_img_path, &export_format, &resolutions),
        result
    )
}

#[test]
fn test_generate_picture_markup() {
    use indoc::indoc;
    let width = 300;
    let height = 200;
    let orig_img_path = "/images/uploads/2020-03-23_20-24-06_393.jpg";
    let result = indoc! {
r#"<picture>
    <source
    srcset="/generated_images/images/uploads/2020-03-23_20-24-06_393_300x200.jpg 1x, /generated_images/images/uploads/2020-03-23_20-24-06_393_450x300.jpg 1.5x, /generated_images/images/uploads/2020-03-23_20-24-06_393_600x400.jpg 2x, /generated_images/images/uploads/2020-03-23_20-24-06_393_900x600.jpg 3x, /generated_images/images/uploads/2020-03-23_20-24-06_393_1200x800.jpg 4x"
    type="image/jpeg"
>
    <img
    src="/generated_images/images/uploads/2020-03-23_20-24-06_393_300x200.jpg"
    width="300"
    height="200"
    alt="Testing image alt"
    
>
</picture>"#,
    };
    pretty_assertions::assert_eq!(
        generate_picture_markup(orig_img_path, width, height, "Testing image alt", None,)
            .expect("picture markup has to be generated"),
        result
    );
}
