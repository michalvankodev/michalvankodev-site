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
}
