use core::fmt;
use std::path::Path;

use image::image_dimensions;
use indoc::formatdoc;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use tracing::{debug, error};

use crate::picture_generator::{
    picture_markup_generator::{generate_picture_markup, should_swap_dimensions},
    resolutions::get_max_resolution,
};

pub const MAX_BLOG_IMAGE_RESOLUTION: (u32, u32) = (1280, 860);

/// A heading extracted from a markdown document, for building tables of contents.
#[derive(Clone, Debug)]
pub struct HeadingToc {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Slugify heading text exactly the way `parse_markdown` does when it injects
/// heading ids, so anchors and TOC links always agree.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
}

/// Extract h2/h3 headings (level, anchor id, text) from a markdown document.
/// Mirrors the id-generation logic inside `parse_markdown`.
pub fn extract_headings(markdown: &str) -> Vec<HeadingToc> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut headings: Vec<HeadingToc> = Vec::new();
    // (level, provided id, first raw Text event, collected display text)
    let mut current: Option<(u8, Option<String>, Option<String>, String)> = None;

    for event in Parser::new_ext(markdown, options) {
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                current = Some((level as u8, id.map(|i| i.to_string()), None, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, _, first_text, collected)) = current.as_mut() {
                    if first_text.is_none() {
                        *first_text = Some(text.to_string());
                    }
                    collected.push_str(&text);
                }
            }
            Event::Code(text) => {
                if let Some((_, _, _, collected)) = current.as_mut() {
                    collected.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, provided_id, first_text, collected)) = current.take() {
                    if level == 2 || level == 3 {
                        // `parse_markdown` slugs the first raw Text event verbatim
                        let id = provided_id.unwrap_or_else(|| slugify(&first_text.unwrap_or_default()));
                        headings.push(HeadingToc {
                            level,
                            id,
                            text: collected.trim().to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    headings
}

enum TextKind {
    Text,
    Heading(Option<String>),
    Code(String),
}

// pub fn parse_markdown(markdown: &str) -> ::askama::Result<String>
#[askama::filter_fn]
pub fn parse_markdown<T: fmt::Display>(
    markdown: T,
    _: &dyn askama::Values,
) -> ::askama::Result<String> {
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
    let mut heading_ended: Option<bool> = None;

    let mds = markdown.to_string();
    let parser = Parser::new_ext(&mds, options).map(|event| match event {
        /*
        Parsing images considers `alt` attribute as inner `Text` event
        Therefore the `[alt]` is rendered in html as subtitle
        and the `[](url "title")` `title` is rendered as `alt` attribute
        */
        Event::Start(Tag::Image {
            link_type: _,
            dest_url,
            title,
            id: _,
        }) => {
            if !dest_url.starts_with("/") {
                return Event::Html(
                    formatdoc!(
                        r#"<img
                          alt="{title}"
                          src="{dest_url}"
                        />"#
                    )
                    .into(),
                );
            }

            // Handle SVG files - don't try to get dimensions (image crate doesn't support SVG)
            if dest_url.to_lowercase().ends_with(".svg") {
                return Event::Html(
                    formatdoc!(
                        r#"<figure>
                            <img src="{dest_url}" alt="{title}">
                            <figcaption>
                        "#
                    )
                    .into(),
                );
            }

            let dev_only_img_path =
                Path::new("static/").join(dest_url.strip_prefix("/").unwrap_or(&dest_url));

            // We need to take the exif rotation into consideration here
            let img_dimensions = {
                let orig_img_dimensions = image_dimensions(&dev_only_img_path).unwrap();
                if should_swap_dimensions(&dev_only_img_path) {
                    (orig_img_dimensions.1, orig_img_dimensions.0)
                } else {
                    orig_img_dimensions
                }
            };
            let (max_width, max_height) = get_max_resolution(
                img_dimensions,
                MAX_BLOG_IMAGE_RESOLUTION.0,
                MAX_BLOG_IMAGE_RESOLUTION.1,
            );

            // Place image into the content with scaled reso to a boundary
            let picture_markup = generate_picture_markup(
                &dest_url, max_width, max_height, &title, None, false,
            )
            .unwrap_or(formatdoc!(
                r#"
                        <img
                          alt="{alt}"
                          src="{src}"
                        />"#,
                alt = title,
                src = dest_url,
            ));
            Event::Html(
                formatdoc!(
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
                let highlighted =
                    highlighted_html_for_string(&text, &syntax_set, syntax_reference, theme)
                        .unwrap();
                Event::Html(highlighted.into())
            }
            TextKind::Heading(provided_id) => {
                let heading_id = provided_id.clone().unwrap_or({
                    text.to_lowercase()
                        .replace(|c: char| !c.is_alphanumeric(), "-")
                });
                debug!("heading_id: {}", heading_id.clone());
                match heading_ended {
                    None => {
                        error!("Heading should have set state");
                        panic!("Heading should have set state");
                    }
                    Some(true) => Event::Html(text),
                    Some(false) => {
                        heading_ended = Some(true);
                        Event::Html(
                            formatdoc!(
                                r##"id="{heading_id}">
                            {text}"##
                            )
                            .into(),
                        )
                    }
                }
            }
            _ => Event::Text(text),
        },
        Event::Start(Tag::Heading {
            level,
            id,
            classes: _,
            attrs: _,
        }) => {
            let id_str = id.map(|id| id.to_string());
            debug!("heading_start: {:?}, level: {}", &id_str, level);
            text_kind = TextKind::Heading(id_str);
            heading_ended = Some(false);
            Event::Html(format!("<{level} ").into())
        }
        Event::Start(_) => event,
        Event::End(TagEnd::Image) => Event::Html("</figcaption></figure>".into()),
        Event::End(TagEnd::CodeBlock) => {
            text_kind = TextKind::Text;
            Event::End(TagEnd::CodeBlock)
        }
        Event::End(TagEnd::Heading(heading_level)) => {
            text_kind = TextKind::Text;
            heading_ended = None;
            Event::End(TagEnd::Heading(heading_level))
        }
        _ => event,
    });

    // Write to String buffer
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(html)
}
