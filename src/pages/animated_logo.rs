use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "assets/animated_logo.html")]
pub struct AnimatedLogoTemplate {}

pub async fn render_animated_logo() -> Result<AnimatedLogoTemplate, StatusCode> {
    Ok(AnimatedLogoTemplate {})
}
