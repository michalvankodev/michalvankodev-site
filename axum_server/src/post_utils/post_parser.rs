use std::path::Path;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use tokio::fs;
use tracing::debug;

use crate::picture_generator::picture_markup_generator::generate_picture_markup;

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
    generate_images: bool,
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
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let body = parse_html(&metadata.content, generate_images);

    let filename = Path::new(path)
        .file_stem()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_str()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .to_owned();

    Ok(ParseResult {
        body,
        metadata: metadata.data,
        slug: filename,
    })
}

pub fn parse_html(markdown: &str, generate_images: bool) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        /*
        Parsing images considers `alt` attribute as inner `Text` event
        Therefore the `[alt]` is rendered in html as subtitle
        and the `[](url "title")` `title` is rendered as `alt` attribute
        */
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            // TODO Get image resolution

            // Place image into the content with scaled reso to a boundary
            let picture_markup =
                generate_picture_markup(&dest_url, 500, 500, &title, generate_images).unwrap_or(
                    format!(
                        r#"
                        <img
                          alt="{alt}"
                          src="{src}"
                        />"#,
                        alt = title,
                        src = dest_url,
                    ),
                );
            // let picture_markup = format!(
            //     r#"
            //             <img
            //               alt="{alt}"
            //               src="{src}"
            //             />"#,
            //     alt = title,
            //     src = dest_url,
            // );

            debug!(
                "Image link_type: {:?} url: {} title: {} id: {}",
                link_type, dest_url, title, id
            );
            Event::Html(
                format!(
                    r#"<figure>
                        {picture_markup}
                        <figcaption>
                    "#,
                )
                .into(),
            )
        }
        Event::Start(_) => event,
        Event::End(TagEnd::Image) => Event::Html("</figcaption></figure>".into()),
        _ => event,
    });

    // Write to String buffer
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}
