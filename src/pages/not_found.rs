use askama::Template;
use axum::{
    extract::OriginalUri,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::info;

use crate::components::site_header::HeaderProps;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundPage {
    title: String,
    header_props: HeaderProps,
}

pub async fn render_not_found(
    OriginalUri(original_uri): OriginalUri,
) -> Result<(StatusCode, impl IntoResponse), StatusCode> {
    info!("{original_uri} not found");
    Ok((
        StatusCode::NOT_FOUND,
        Html(
            NotFoundPage {
                title: "This page does not exists".to_owned(),
                header_props: HeaderProps::default(),
            }
            .render()
            .unwrap(),
        ),
    ))
}
