use askama::Template;
use axum::http::StatusCode;

use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::HeaderProps,
    },
    tag_list::get_popular_blog_tags,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    site_footer: SiteFooter,
    header_props: HeaderProps,
    blog_tags: Vec<String>,
}

pub async fn render_index() -> Result<IndexTemplate, StatusCode> {
    let site_footer = tokio::spawn(render_site_footer());
    let blog_tags = tokio::spawn(get_popular_blog_tags());

    let blog_tags = blog_tags
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)??;

    let site_footer = site_footer
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(IndexTemplate {
        site_footer,
        header_props: HeaderProps::default(),
        blog_tags,
    })
}
