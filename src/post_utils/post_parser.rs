use std::path::Path;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
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

#[derive(Clone)]
pub struct ParseResult<Metadata> {
    pub body: String,
    pub metadata: Metadata,
    pub slug: String,
}

pub async fn parse_post<Metadata: DeserializeOwned>(
    path: &str,
) -> Result<ParseResult<Metadata>, StatusCode> {
    let file_contents = fs::read_to_string(path)
        .await
        // TODO Proper reasoning for an error
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let matter = Matter::<YAML>::new();
    let parsed = matter
        .parse::<Metadata>(&file_contents)
        .map_err(|err| {
            tracing::error!("Failed to parse metadata: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let metadata = parsed.data.ok_or_else(|| {
        tracing::error!("Missing front matter in {path}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let filename = Path::new(path)
        .file_stem()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_str()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_owned();

    Ok(ParseResult {
        body: parsed.content,
        metadata,
        slug: filename,
    })
}
