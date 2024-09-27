use askama::Template;
use axum::http::StatusCode;

use crate::{
    components::site_header::HeaderProps,
    post_utils::{post_listing::get_post_list, post_parser::ParseResult},
    projects::project_model::ProjectMetadata,
};

#[derive(Template)]
#[template(path = "project_list.html")]
pub struct ProjectListTemplate {
    pub title: String,
    pub project_list: Vec<ParseResult<ProjectMetadata>>,
    pub header_props: HeaderProps,
}

pub async fn render_projects_list() -> Result<ProjectListTemplate, StatusCode> {
    let mut project_list = get_post_list::<ProjectMetadata>("../_projects").await?;

    project_list.sort_by_key(|post| post.slug.to_string());
    project_list.retain(|project| project.metadata.displayed);
    project_list.reverse();

    Ok(ProjectListTemplate {
        title: "Showcase".to_owned(),
        header_props: HeaderProps::default(),
        project_list,
    })
}
