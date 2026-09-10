use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::post_utils::post_parser::deserialize_date;

/// Article cover image, independent from the listing `thumbnail`.
///
/// Front matter accepts three shapes:
/// - `cover: /images/uploads/foo.jpg` — explicit cover, may differ from the
///   thumbnail
/// - `cover: false` — no cover figure on the article page (thumbnail is
///   still used for listing previews and og:image)
/// - field omitted — falls back to `thumbnail` (backward compatible: every
///   post written before `cover` existed keeps its cover)
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum Cover {
    Disabled,
    #[default]
    Thumbnail,
    Image(String),
}

pub fn deserialize_cover<'de, D>(deserializer: D) -> Result<Cover, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Input {
        Path(String),
        Flag(bool),
    }

    match Input::deserialize(deserializer)? {
        Input::Path(path) => Ok(Cover::Image(path)),
        Input::Flag(false) => Ok(Cover::Disabled),
        Input::Flag(true) => Ok(Cover::Thumbnail),
    }
}

pub const BLOG_POST_PATH: &str = "_posts/blog";

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")] // Optional, this converts enum variants to lowercase
pub enum Segment {
    Blog,
    Broadcasts,
    Featured,
    Cookbook,
}

#[derive(Deserialize, Debug)]
pub struct BlogPostMetadata {
    pub title: String,
    pub segments: Vec<Segment>,
    pub published: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    pub thumbnail: Option<String>,
    /// Article cover (see `Cover`). Missing field falls back to `thumbnail`.
    #[serde(default, deserialize_with = "deserialize_cover")]
    pub cover: Cover,
    pub tags: Vec<String>,
    /// Hand-written summary used for og:description/twitter:description and
    /// feed summaries. Optional — a plain-text excerpt is derived from the
    /// post body when absent (see post_utils::post_description).
    #[serde(default)]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;

    #[test]
    fn cover_path_gives_explicit_image() {
        let cover = deserialize_cover(
            serde_json::Value::String("/images/uploads/cover.jpg".into()).into_deserializer(),
        )
        .unwrap();
        assert_eq!(cover, Cover::Image("/images/uploads/cover.jpg".into()));
    }

    #[test]
    fn cover_false_disables_the_cover() {
        let cover =
            deserialize_cover(serde_json::Value::Bool(false).into_deserializer()).unwrap();
        assert_eq!(cover, Cover::Disabled);
    }

    #[test]
    fn cover_true_behaves_like_the_default() {
        let cover = deserialize_cover(serde_json::Value::Bool(true).into_deserializer()).unwrap();
        assert_eq!(cover, Cover::Thumbnail);
    }

    #[test]
    fn missing_cover_field_falls_back_to_thumbnail() {
        let metadata: BlogPostMetadata = serde_json::from_str(
            r#"{
                "title": "t",
                "segments": ["blog"],
                "published": true,
                "date": "2023-06-24T16:34:45Z",
                "thumbnail": "/images/uploads/thumb.jpg",
                "tags": []
            }"#,
        )
        .unwrap();
        assert_eq!(metadata.cover, Cover::Thumbnail);
    }
}
