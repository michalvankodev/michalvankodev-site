use crate::{pages::project::ProjectMetadata, post_list::get_post_list, post_parser::ParseResult};
use axum::http::StatusCode;

pub async fn get_featured_projects() -> Result<Vec<ParseResult<ProjectMetadata>>, StatusCode> {
    let project_list = get_post_list::<ProjectMetadata>("../_projects").await?;

    let featured_projects = project_list
        .into_iter()
        .filter(|post| post.metadata.featured)
        .collect();

    Ok(featured_projects)
}
