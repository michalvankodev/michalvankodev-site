use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "assets/animated_logo.html")]
pub struct AnimatedLogoTemplate {}

pub async fn render_animated_logo() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html(AnimatedLogoTemplate {}.render().unwrap()))
}
