use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminPageTemplate {}

pub async fn render_admin() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html(
        AdminPageTemplate {}
            .render()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
    ))
}
