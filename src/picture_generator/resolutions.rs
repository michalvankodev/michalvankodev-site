pub fn get_max_resolution_with_crop(
    (orig_width, orig_height): (u32, u32),
    width: u32,
    height: u32,
) -> (u32, u32) {
    let width_scale = orig_width as f32 / width as f32;
    let height_scale = orig_height as f32 / height as f32;

    let scale = width_scale.min(height_scale);
    (
        (width as f32 * scale) as u32,
        (height as f32 * scale) as u32,
    )
}

#[test]
fn test_get_max_resolution_with_crop() {
    assert_eq!(
        get_max_resolution_with_crop((320, 200), 320, 200),
        (320, 200),
        "Original size fits"
    );
    // THINK: Real curious if this is what I want to do. Rather than use CSS to `object-cover` original image size
    assert_eq!(
        get_max_resolution_with_crop((200, 200), 300, 200),
        (200, 133),
        "Image has to be smaller"
    );

    assert_eq!(
        get_max_resolution_with_crop((1000, 1000), 200, 100),
        (1000, 500),
        "width is maxed"
    );
    assert_eq!(
        get_max_resolution_with_crop((1000, 1000), 100, 200),
        (500, 1000),
        "height is maxed"
    );
    assert_eq!(
        get_max_resolution_with_crop((300, 200), 600, 500),
        (240, 200),
        "image has to be scaled down"
    );
}

pub fn get_max_resolution(
    (orig_width, orig_height): (u32, u32),
    max_width: u32,
    max_height: u32,
) -> (u32, u32) {
    // If the original dimensions are within the max dimensions, return them as is
    if orig_width <= max_width && orig_height <= max_height {
        return (orig_width, orig_height);
    }

    let width_scale = max_width as f32 / orig_width as f32;
    let height_scale = max_height as f32 / orig_height as f32;

    // Determine the scaling factor to ensure the image fits within the bounds
    let scale = width_scale.min(height_scale);

    (
        (orig_width as f32 * scale).round() as u32,
        (orig_height as f32 * scale).round() as u32,
    )
}

#[test]
fn test_get_max_resolution() {
    assert_eq!(
        get_max_resolution((999, 675), 1000, 800),
        (999, 675),
        "Original size fits"
    );
    assert_eq!(
        get_max_resolution((1100, 400), 1000, 800),
        (1000, 364),
        "Image should be resized to fit within max dimensions"
    );
    assert_eq!(
        get_max_resolution((1100, 1200), 1000, 800),
        (733, 800),
        "Image should be resized to fit within max dimensions"
    );
    assert_eq!(
        get_max_resolution((1100, 800), 1000, 800),
        (1000, 727),
        "Image should be resized to fit within max dimensions"
    );
}
