use askama::filters::format;
use axum;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pages;
mod post_parser;
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
    let app = router::get_router();
    // run our app with hyper, listening globally on port 3080
    let port = std::option_env!("PORT").unwrap_or("3080");
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// TODO displaying Image from netlify CDN
