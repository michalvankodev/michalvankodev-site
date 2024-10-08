use axum::http::StatusCode;

use crate::post_utils::{post_listing::get_post_list, post_parser::ParseResult};

use super::project_model::ProjectMetadata;

pub async fn get_featured_projects() -> Result<Vec<ParseResult<ProjectMetadata>>, StatusCode> {
    let project_list = get_post_list::<ProjectMetadata>("_projects").await?;

    let featured_projects = project_list
        .into_iter()
        .filter(|post| post.metadata.featured)
        .collect();

    Ok(featured_projects)
}
