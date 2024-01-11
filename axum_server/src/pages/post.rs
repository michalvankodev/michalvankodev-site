use askama::Template;
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::post_parser::{deserialize_date, parse_post};

#[derive(Deserialize, Debug)]
pub struct PostMetadata {
    pub layout: String,
    pub title: String,
    pub segments: Vec<String>,
    pub published: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    pub thumbnail: String,
    pub tags: Vec<String>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub content: String,
}

pub async fn render_post(Path(post_id): Path<String>) -> Result<PostTemplate, StatusCode> {
    let path = format!("../_posts/blog/{}.md", post_id);
    let parsed = parse_post::<PostMetadata>(&path).await?;
    Ok(PostTemplate {
        title: parsed.metadata.title,
        content: parsed.content,
    })
}
