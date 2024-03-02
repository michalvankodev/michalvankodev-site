use std::path::Path;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
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
    pub body: String,
    pub metadata: Metadata,
    pub slug: String,
}

pub async fn parse_post<'de, Metadata: DeserializeOwned>(
    path: &str,
) -> Result<ParseResult<Metadata>, StatusCode> {
    let file_contents = fs::read_to_string(path)
        .await
        // TODO Proper reasoning for an error
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let matter = Matter::<YAML>::new();
    let metadata = matter
        .parse_with_struct::<Metadata>(&file_contents)
        .ok_or_else(|| {
            tracing::error!("Failed to parse metadata");
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    let body = parse_html(&metadata.content);

    let filename = Path::new(path)
        .file_stem()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_str()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_owned();

    return Ok(ParseResult {
        body,
        metadata: metadata.data,
        slug: filename,
    });
}

fn parse_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&markdown, options).map(|event| match event {
        Event::Start(ref tag) => match tag {
            // Parsing images considers `alt` attribute as inner `Text` event
            // Therefore the `[alt]` is rendered in html as subtitle
            // and the `[](url "title")` `title` is rendered as `alt` attribute
            Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            } => {
                println!(
                    "Image link_type: {:?} url: {} title: {} id: {}",
                    link_type, dest_url, title, id
                );
                // TODO src set
                Event::Html(
                    format!(
                        r#"<figure>
                            <img
                              alt="{alt}"
                              src="{src}"
                            />
                            <figcaption>
                        "#,
                        alt = title,
                        src = dest_url,
                    )
                    .into(),
                )
            }
            _ => event,
        },
        Event::End(TagEnd::Image) => Event::Html("</figcaption></figure>".into()),
        _ => event,
    });

    // Write to String buffer.
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    return html;
}
