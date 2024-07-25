use crate::{components::site_header::Link, filters};
use askama::Template;
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokio::try_join;

use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::HeaderProps,
    },
    post_parser::{deserialize_date, parse_post},
};

pub const BLOG_POST_PATH: &str = "../_posts/blog";

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
    let path = format!("../_posts/blog/{}.md", post_id);
    let parse_post = parse_post::<PostMetadata>(&path);
    let (site_footer, parsed) = try_join!(render_site_footer(), parse_post)?;

    Ok(PostTemplate {
        title: parsed.metadata.title,
        date: parsed.metadata.date,
        tags: parsed.metadata.tags,
        body: parsed.body,
        site_footer,
        header_props: HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
    })
}
