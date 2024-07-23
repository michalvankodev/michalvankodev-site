use askama::Template;
use axum::http::StatusCode;
use tokio::try_join;

use crate::filters;
use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::HeaderProps,
    },
    featured_posts::get_featured_posts,
    featured_projects::get_featured_projects,
    post_parser::ParseResult,
    tag_list::get_popular_blog_tags,
};

use super::post::PostMetadata;
use super::project::ProjectMetadata;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    site_footer: SiteFooter,
    header_props: HeaderProps,
    blog_tags: Vec<String>,
    featured_posts: Vec<ParseResult<PostMetadata>>,
    featured_projects: Vec<ParseResult<ProjectMetadata>>,
}

pub async fn render_index() -> Result<IndexTemplate, StatusCode> {
    let (site_footer, blog_tags, featured_posts, featured_projects) = try_join!(
        render_site_footer(),
        get_popular_blog_tags(),
        get_featured_posts(),
        get_featured_projects()
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(IndexTemplate {
        site_footer,
        header_props: HeaderProps::default(),
        blog_tags,
        featured_posts,
        featured_projects,
    })
}
