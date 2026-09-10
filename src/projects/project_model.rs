use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectMetadata {
    pub title: String,
    pub classification: String,
    pub displayed: bool,
    pub cover_image: Option<String>,
    pub tags: Vec<String>,
    pub featured: bool,
    pub link: Option<String>,
}

/// Projects are slugged `YYYY-MM-DD-title` — derive the year for
/// machine-voice metadata (mono footer on plates and rows).
pub fn year_from_slug(slug: &str) -> Option<&str> {
    let prefix = slug.get(..4)?;
    prefix
        .chars()
        .all(|c| c.is_ascii_digit())
        .then_some(prefix)
}

pub fn translate_classification(classification: &str) -> &str {
    match classification {
        "webapp" => "Web application",
        "website" => "Web site",
        "presentation" => "Presentation",
        "videogame" => "Video game",
        "embedded" => "Embedded system",
        any => any,
    }
}

/// Tags minus the ones that only restate the classification (e.g. `#webapp`
/// next to "web application") — the mono footer states classification once,
/// tags should add information.
pub fn display_tags<'a>(classification: &str, tags: &'a [String]) -> Vec<&'a str> {
    let classification_display = translate_classification(classification);
    tags.iter()
        .map(String::as_str)
        .filter(|tag| {
            !tag.eq_ignore_ascii_case(classification)
                && !tag.eq_ignore_ascii_case(classification_display)
        })
        .collect()
}
