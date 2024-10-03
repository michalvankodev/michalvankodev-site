use askama::Template;
use axum::{extract::OriginalUri, http::StatusCode};
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
) -> Result<(StatusCode, NotFoundPage), StatusCode> {
    info!("{original_uri} not found");
    Ok((
        StatusCode::NOT_FOUND,
        NotFoundPage {
            title: "This page does not exists".to_owned(),
            header_props: HeaderProps::default(),
        },
    ))
}
