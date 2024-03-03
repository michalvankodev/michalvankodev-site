use crate::filters;
use askama::Template;
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::HeaderProps,
    },
    post_parser::{deserialize_date, parse_post},
};

#[derive(Deserialize, Debug)]
pub struct PostMetadata {
    pub layout: String,
    pub title: String,
    pub segments: Vec<String>,
    pub published: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    pub thumbnail: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub title: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub site_footer: SiteFooter,
    pub header_props: HeaderProps,
}

pub async fn render_post(Path(post_id): Path<String>) -> Result<PostTemplate, StatusCode> {
    let site_footer = tokio::spawn(render_site_footer());
    let path = format!("../_posts/blog/{}.md", post_id);
    let parsed = parse_post::<PostMetadata>(&path).await?;

    let site_footer = site_footer
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(PostTemplate {
        title: parsed.metadata.title,
        date: parsed.metadata.date,
        tags: parsed.metadata.tags,
        body: parsed.body,
        site_footer,
        header_props: HeaderProps::default(),
    })
}
