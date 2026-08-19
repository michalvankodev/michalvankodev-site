use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use pulldown_cmark::{Event, Options, Parser};
use serde::Serialize;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH},
    components::site_header::HeaderProps,
    post_utils::post_listing::get_post_list,
    post_utils::post_parser::ParseResult,
};

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchPageTemplate {
    pub title: String,
    pub header_props: HeaderProps,
}

pub async fn render_search() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html(
        SearchPageTemplate {
            title: "Search".to_string(),
            header_props: HeaderProps::default(),
        }
        .render()
        .unwrap(),
    ))
}

#[derive(Serialize)]
pub struct SearchEntry {
    pub title: String,
    pub segment: String,
    pub slug: String,
    pub date: String,
    pub tags: Vec<String>,
    pub excerpt: String,
}

/// First ~220 characters of plain text of a markdown document.
fn plain_text_excerpt(markdown: &str) -> String {
    let mut excerpt = String::new();
    for event in Parser::new_ext(markdown, Options::empty()) {
        match event {
            Event::Text(text) | Event::Code(text) => {
                excerpt.push_str(&text);
                if excerpt.chars().count() >= 220 {
                    break;
                }
            }
            Event::SoftBreak | Event::HardBreak | Event::End(_) => excerpt.push(' '),
            _ => {}
        }
    }
    let trimmed: String = excerpt.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut result = String::new();
    for (i, c) in trimmed.chars().enumerate() {
        if i >= 220 {
            result.push('…');
            break;
        }
        result.push(c);
    }
    result
}

/// JSON index consumed by the client-side search on `/search`.
pub async fn search_index() -> Result<impl IntoResponse, StatusCode> {
    let posts = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;
    let mut entries: Vec<SearchEntry> = posts
        .into_iter()
        .map(|post: ParseResult<BlogPostMetadata>| {
            let segment = if post
                .metadata
                .segments
                .iter()
                .any(|segment| matches!(segment, Segment::Blog))
            {
                "blog"
            } else if post
                .metadata
                .segments
                .iter()
                .any(|segment| matches!(segment, Segment::Broadcasts))
            {
                "broadcasts"
            } else {
                "blog"
            };
            SearchEntry {
                title: post.metadata.title,
                segment: segment.to_string(),
                slug: post.slug,
                date: post.metadata.date.format("%Y-%m-%d").to_string(),
                tags: post.metadata.tags,
                excerpt: plain_text_excerpt(&post.body),
            }
        })
        .collect();
    entries.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(Json(entries))
}
