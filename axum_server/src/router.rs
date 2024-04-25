use crate::{
    feed::render_rss_feed,
    pages::{
        contact::render_contact, index::render_index, post::render_post,
        post_list::render_post_list,
    },
};
use axum::{extract::MatchedPath, http::Request, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(render_index))
        .route("/blog", get(render_post_list))
        .route("/blog/tags/:tag", get(render_post_list))
        .route("/blog/:post_id", get(render_post))
        .route("/contact", get(render_contact))
        .route("/feed.xml", get(render_rss_feed))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
}
