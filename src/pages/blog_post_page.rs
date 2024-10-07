use askama::Template;
use axum::extract::OriginalUri;
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};

use crate::blog_posts::blog_post_model::BLOG_POST_PATH;
use crate::{
    blog_posts::blog_post_model::BlogPostMetadata, components::site_header::Link, filters,
    post_utils::post_parser::parse_post,
};

use crate::components::site_header::HeaderProps;

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate {
    pub title: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub segment: String,
    pub header_props: HeaderProps,
    pub slug: String,
    pub thumbnail: Option<String>,
}

pub async fn render_blog_post(
    Path(post_id): Path<String>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<BlogPostTemplate, StatusCode> {
    let path = format!("{}/{}.md", BLOG_POST_PATH, post_id);
    let parse_post = parse_post::<BlogPostMetadata>(&path);
    let parsed = parse_post.await?;
    let segment = if original_uri.to_string().starts_with("/blog") {
        "blog"
    } else if original_uri.to_string().starts_with("/broadcasts") {
        "broadcasts"
    } else {
        "blog"
    };

    let header_props = match segment {
        "blog" => HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
        "broadcasts" => HeaderProps::with_back_link(Link {
            href: "/broadcasts".to_string(),
            label: "All broadcasts".to_string(),
        }),
        _ => HeaderProps::default(),
    };

    Ok(BlogPostTemplate {
        title: parsed.metadata.title,
        date: parsed.metadata.date,
        tags: parsed.metadata.tags,
        body: parsed.body,
        slug: parsed.slug,
        segment: segment.to_string(),
        thumbnail: parsed.metadata.thumbnail,
        header_props,
    })
}
