use askama::Template;
use axum::http::StatusCode;

use crate::filters;
use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::HeaderProps,
    },
    featured_posts::get_featured_posts,
    post_parser::ParseResult,
    tag_list::get_popular_blog_tags,
};

use super::post::PostMetadata;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    site_footer: SiteFooter,
    header_props: HeaderProps,
    blog_tags: Vec<String>,
    featured_posts: Vec<ParseResult<PostMetadata>>,
}

pub async fn render_index() -> Result<IndexTemplate, StatusCode> {
    let site_footer = tokio::spawn(render_site_footer());
    let blog_tags = tokio::spawn(get_popular_blog_tags());
    let featured_posts = tokio::spawn(get_featured_posts());

    let blog_tags = blog_tags
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;

    let site_footer = site_footer
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let featured_posts = featured_posts
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;

    Ok(IndexTemplate {
        site_footer,
        header_props: HeaderProps::default(),
        blog_tags,
        featured_posts,
    })
}
