use axum::{self};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod blog_posts;
mod components;
mod feed;
mod filters;
mod pages;
mod picture_generator;
mod post_utils;
mod projects;
mod router;
// mod template;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_server=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = router::get_router()
        .nest_service("/styles", ServeDir::new("styles"))
        .nest_service("/images", ServeDir::new("static/images"))
        .nest_service("/fonts", ServeDir::new("static/fonts"))
        .nest_service("/files", ServeDir::new("static/files"))
        .nest_service("/generated_images", ServeDir::new("generated_images"))
        .nest_service("/egg-fetcher", ServeDir::new("static/egg-fetcher"))
        .nest_service("/svg", ServeDir::new("static/svg"))
        .nest_service("/config.yml", ServeDir::new("static/resources/config.yml")) // Decap CMS config
        .nest_service("/resources", ServeDir::new("static/resources"))
        .nest_service("/robots.txt", ServeDir::new("robots.txt"));

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    // run our app with hyper, listening globally on port 3080
    let port = std::option_env!("PORT").unwrap_or("3080");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("axum_server listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}

// TODO Socials
// - fotos
// THINK deploy to alula? rather then katelyn? can be change whenever
//
// TODO view page transitions
// TODO cookbook
// TODO remove m-logo-svg from justfile and mention it in some article!!! WRITE SOME NEW ARTICLES
