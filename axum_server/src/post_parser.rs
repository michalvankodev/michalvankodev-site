use std::path::Path;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use comrak::{
    format_html,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, Options,
};
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
    let mut options = Options::default();
    options.parse.smart = true;
    options.parse.relaxed_autolinks = true;
    options.extension.strikethrough = true;
    // options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = false;
    options.extension.description_lists = false;
    options.extension.multiline_block_quotes = false;
    options.extension.shortcodes = true;
    options.render.hardbreaks = true;
    options.render.github_pre_lang = true;
    options.render.full_info_string = true;
    options.render.unsafe_ = true;

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let root = parse_document(&arena, markdown, &options);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::Text(ref mut _text) => {
            // let orig = std::mem::replace(text, String::new());
            // *text = orig.replace("my", "your");
        }
        _ => (),
    });

    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();
    return String::from_utf8(html).unwrap();
}
