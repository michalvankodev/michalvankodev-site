use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::post_utils::post_parser::deserialize_date;

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
    pub tags: Vec<String>,
    /// Hand-written summary used for og:description/twitter:description and
    /// feed summaries. Optional — a plain-text excerpt is derived from the
    /// post body when absent (see post_utils::post_description).
    #[serde(default)]
    pub description: Option<String>,
}
