use askama::Template;

use crate::{
    blog_posts::blog_post_model::BlogPostMetadata, components::site_header::HeaderProps, filters,
    post_utils::post_parser::ParseResult, projects::project_model::ProjectMetadata,
};

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub title: String,
    pub og_title: String,
    pub segment: String,
    pub posts: Vec<ParseResult<BlogPostMetadata>>,
    pub header_props: HeaderProps,
    pub tags: Vec<String>,
    pub featured_projects: Vec<ParseResult<ProjectMetadata>>,
    pub current_url: String,
}
