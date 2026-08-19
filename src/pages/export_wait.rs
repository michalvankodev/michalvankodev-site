use axum::http::StatusCode;

/// Long-polls until the image generation queue is fully drained (nothing queued
/// or running). Used by the SSG export flow: the crawler first warms all pages
/// (which enqueues image jobs), then waits on this endpoint before crawling the
/// site into `dist/` — guaranteeing every generated image exists and is complete.
pub async fn export_wait() -> StatusCode {
    tracing::info!("Waiting for image generation queue to drain...");
    crate::picture_generator::image_jobs::wait_until_idle().await;
    tracing::info!("Image generation queue is idle");
    StatusCode::OK
}
