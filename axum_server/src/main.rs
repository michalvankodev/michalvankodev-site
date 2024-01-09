use axum::{
    extract::{MatchedPath, Path},
    http::{Request, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use markdown::{to_html_with_options, CompileOptions, Constructs, Options, ParseOptions};
use serde::{Deserialize, Deserializer};
use tokio::fs;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/blog/:post_id", get(parse_post))
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
        );

    // run our app with hyper, listening globally on port 3080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn parse_post(Path(post_id): Path<String>) -> Result<Html<String>, StatusCode> {
    let path = format!("../_posts/blog/{}", post_id);
    let contents = fs::read_to_string(path).await;

    let raw_content = match contents {
        Err(_reason) => {
            // TODO find the real reason
            return Err(StatusCode::NOT_FOUND);
        }
        Ok(content) => content,
    };

    let markdown_options = Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Default::default()
            },
            ..Default::default()
        },
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..Default::default()
        },
        ..Default::default()
    };

    #[derive(Deserialize, Debug)]
    struct Metadata {
        layout: String,
        title: String,
        segments: Vec<String>,
        published: bool,
        #[serde(deserialize_with = "deserialize_date")]
        date: DateTime<Utc>,
        thumbnail: String,
        tags: Vec<String>,
    }

    let matter = Matter::<YAML>::new();
    let metadata = matter.parse_with_struct::<Metadata>(&raw_content);

    // Deserialize JSON into MyData struct

    // Print the entire struct using the Debug trait
    println!("{:?}", metadata.unwrap().data);

    let parsed = to_html_with_options(&raw_content, &markdown_options);

    let content = match parsed {
        Err(reason) => {
            tracing::error!(reason);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(content) => content,
    };

    // TODO Parse file
    return Ok(Html(content));
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&date_str) {
        Ok(datetime) => Ok(datetime.with_timezone(&Utc)),
        Err(err) => Err(serde::de::Error::custom(format!(
            "Error parsing date: {}",
            err
        ))),
    }
}

// TODO Port from env variable
// TODO templating system
// TODO simple Logging
// TODO parse md files
