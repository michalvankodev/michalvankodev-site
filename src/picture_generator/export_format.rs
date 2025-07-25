#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExportFormat {
    Jpeg,
    Avif,
    Svg,
    Png,
}

impl ExportFormat {
    pub fn get_extension(&self) -> &str {
        match self {
            ExportFormat::Jpeg => "jpg",
            ExportFormat::Avif => "avif",
            ExportFormat::Svg => "svg",
            ExportFormat::Png => "png",
        }
    }
    pub fn get_type(&self) -> &str {
        match self {
            ExportFormat::Jpeg => "image/jpeg",
            ExportFormat::Avif => "image/avif",
            ExportFormat::Svg => "image/svg+xml",
            ExportFormat::Png => "image/png",
        }
    }
}
