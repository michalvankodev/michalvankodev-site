use askama::Template;
use axum::http::StatusCode;
use tokio::try_join;

use crate::{
    blog_posts::{
        blog_post_model::BlogPostMetadata, featured_blog_posts::get_featured_blog_posts,
        tag_list::get_popular_tags,
    },
    components::site_header::HeaderProps,
    filters,
    post_utils::post_parser::ParseResult,
    projects::{featured_projects::get_featured_projects, project_model::ProjectMetadata},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    header_props: HeaderProps,
    blog_tags: Vec<String>,
    featured_blog_posts: Vec<ParseResult<BlogPostMetadata>>,
    featured_projects: Vec<ParseResult<ProjectMetadata>>,
}

pub async fn render_index() -> Result<IndexTemplate, StatusCode> {
    let (blog_tags, featured_blog_posts, featured_projects) = try_join!(
        get_popular_tags(Some("blog".to_string())),
        get_featured_blog_posts(),
        get_featured_projects()
    )?;

    Ok(IndexTemplate {
        header_props: HeaderProps::default(),
        blog_tags,
        featured_blog_posts,
        featured_projects,
    })
}
