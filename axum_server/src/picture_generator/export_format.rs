
#[derive(Debug, PartialEq)]
pub enum ExportFormat {
    JPG,
    AVIF,
    SVG,
    PNG,
}

impl ExportFormat {
    pub fn get_extension(&self) -> &str {
        match self {
            ExportFormat::JPG => "jpg",
            ExportFormat::AVIF => "avif",
            ExportFormat::SVG => "svg",
            ExportFormat::PNG => "png",
        }
    }
    pub fn get_type(&self) -> &str {
        match self {
            ExportFormat::JPG => "image/jpeg",
            ExportFormat::AVIF => "image/avif",
            ExportFormat::SVG => "image/svg+xml",
            ExportFormat::PNG => "image/png",
        }
    }
}
