use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectMetadata {
    pub title: String,
    pub description: String,
    pub classification: String,
    pub displayed: bool,
    pub cover_image: Option<String>,
    pub tags: Vec<String>,
    pub featured: bool,
    pub link: Option<String>,
}

pub fn translate_classification(classification: &str) -> &str {
    match classification {
        "webapp" => "Web application",
        "website" => "Web site",
        "presentation" => "Presentation",
        "videogame" => "Video game",
        any => any,
    }
}
