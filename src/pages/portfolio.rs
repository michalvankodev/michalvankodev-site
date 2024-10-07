use askama::Template;
use axum::http::StatusCode;
use serde::Deserialize;

use crate::{
    components::site_header::HeaderProps,
    filters,
    post_utils::{
        post_listing::get_post_list,
        post_parser::{parse_post, ParseResult},
    },
    projects::project_model::ProjectMetadata,
};

#[derive(Deserialize, Debug)]
pub struct PortfolioPageModel {
    pub title: String,
    // TODO work_history
    // TODO education
}

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate {
    pub title: String,
    pub body: String,
    pub project_list: Vec<ParseResult<ProjectMetadata>>,
    pub header_props: HeaderProps,
}

pub async fn render_portfolio() -> Result<PortfolioTemplate, StatusCode> {
    let portfolio = parse_post::<PortfolioPageModel>("_pages/portfolio.md").await?;

    let mut project_list = get_post_list::<ProjectMetadata>("_projects").await?;

    project_list.sort_by_key(|post| post.slug.to_string());
    project_list.retain(|project| project.metadata.displayed);
    project_list.reverse();

    Ok(PortfolioTemplate {
        title: "Portfolio".to_owned(),
        body: portfolio.body,
        header_props: HeaderProps::default(),
        project_list,
    })
}
