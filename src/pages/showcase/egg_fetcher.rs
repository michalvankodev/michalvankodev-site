use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::components::site_header::HeaderProps;

#[derive(Template)]
#[template(path = "egg_fetcher_page.html")]
pub struct EggFetcherShowcaseTemplate {
    header_props: HeaderProps,
}

pub async fn render_egg_fetcher() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html(
        EggFetcherShowcaseTemplate {
            header_props: HeaderProps::default(),
        }
        .render()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
    ))
}
