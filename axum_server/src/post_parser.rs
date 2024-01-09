use axum::{extract::Path, http::StatusCode, response::Html};
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use markdown::{to_html_with_options, CompileOptions, Constructs, Options, ParseOptions};
use serde::{Deserialize, Deserializer};
use tokio::fs;

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

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub layout: String,
    pub title: String,
    pub segments: Vec<String>,
    pub published: bool,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    pub thumbnail: String,
    pub tags: Vec<String>,
}

pub async fn parse_post(Path(post_id): Path<String>) -> Result<Html<String>, StatusCode> {
    let path = format!("../_posts/blog/{}.md", post_id);
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

    let matter = Matter::<YAML>::new();
    let _metadata = matter.parse_with_struct::<Metadata>(&raw_content);
    let parsed = to_html_with_options(&raw_content, &markdown_options);

    let content = match parsed {
        Err(reason) => {
            tracing::error!(reason);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(content) => content,
    };

    return Ok(Html(content));
}
