use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::post_utils::post_parser::deserialize_date;

pub const BLOG_POST_PATH: &str = "_posts/blog";

#[derive(Deserialize, Debug)]
pub struct BlogPostMetadata {
    pub title: String,
    pub segments: Vec<String>,
    pub published: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    pub thumbnail: Option<String>,
    pub tags: Vec<String>,
}
