use std::path::Path;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use gray_matter::{engine::YAML, Matter};
use image::image_dimensions;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tokio::fs;
use tracing::debug;

use crate::picture_generator::{
    picture_markup_generator::generate_picture_markup, resolutions::get_max_resolution,
};

pub const MAX_BLOG_IMAGE_RESOLUTION: (u32, u32) = (1280, 860);

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

enum TextKind {
    Text,
    Code(String),
}

pub fn parse_html(markdown: &str, generate_images: bool) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut text_kind = TextKind::Text;
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes.get("InspiredGitHub").unwrap();

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
            if !dest_url.starts_with("/") {
                return Event::Html(
                    format!(
                        r#"<img
                          alt="{title}"
                          src="{dest_url}"
                        />"#
                    )
                    .into(),
                );
            }

            let dev_only_img_path =
                Path::new("../static/").join(dest_url.strip_prefix("/").unwrap_or(&dest_url));
            let img_dimensions = image_dimensions(&dev_only_img_path).unwrap();

            let (max_width, max_height) = get_max_resolution(
                img_dimensions,
                MAX_BLOG_IMAGE_RESOLUTION.0,
                MAX_BLOG_IMAGE_RESOLUTION.1,
            );

            // Place image into the content with scaled reso to a boundary
            let picture_markup =
                generate_picture_markup(&dest_url, max_width, max_height, &title, generate_images)
                    .unwrap_or(format!(
                        r#"
                        <img
                          alt="{alt}"
                          src="{src}"
                        />"#,
                        alt = title,
                        src = dest_url,
                    ));
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
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            text_kind = TextKind::Code(lang.to_string());
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
        }
        Event::Text(text) => match &text_kind {
            TextKind::Code(lang) => {
                // TODO Check https://github.com/trishume/syntect/pull/535 for typescript support
                let lang = if ["ts".to_string(), "typescript".to_string()].contains(lang) {
                    "javascript"
                } else {
                    lang
                };
                let syntax_reference = syntax_set
                    .find_syntax_by_token(lang)
                    .unwrap_or(syntax_set.find_syntax_plain_text());
                syntax_set.syntaxes().iter().for_each(|sr| {
                    debug!("{}", sr.name);
                });
                let highlighted =
                    highlighted_html_for_string(&text, &syntax_set, syntax_reference, theme)
                        .unwrap();
                Event::Html(highlighted.into())
            }
            _ => Event::Text(text),
        },
        Event::Start(_) => event,
        Event::End(TagEnd::Image) => Event::Html("</figcaption></figure>".into()),
        Event::End(TagEnd::CodeBlock) => {
            text_kind = TextKind::Text;
            Event::End(TagEnd::CodeBlock)
        }
        _ => event,
    });

    // Write to String buffer
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}
