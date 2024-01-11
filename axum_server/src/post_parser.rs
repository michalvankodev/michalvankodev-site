use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use markdown::{to_html_with_options, CompileOptions, Constructs, Options, ParseOptions};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use tokio::fs;

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
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

pub struct ParseResult<Metadata> {
    pub content: String,
    pub metadata: Metadata,
}

pub async fn parse_post<'de, Metadata: DeserializeOwned>(
    path: &str,
) -> Result<ParseResult<Metadata>, StatusCode> {
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

    let parsed = to_html_with_options(&raw_content, &markdown_options);
    let content = match parsed {
        Err(reason) => {
            tracing::error!(reason);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(content) => content,
    };

    let matter = Matter::<YAML>::new();
    let parsed_metadata = matter.parse_with_struct::<Metadata>(&raw_content);

    let metadata = match parsed_metadata {
        None => {
            tracing::error!("Failed to parse metadata");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Some(metadata) => metadata,
    };

    return Ok(ParseResult {
        content,
        metadata: metadata.data,
    });
}
