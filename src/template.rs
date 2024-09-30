pub use askama::*;
use axum::http::Response;

pub fn into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => Html(body),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
