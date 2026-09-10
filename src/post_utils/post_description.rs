//! Plain-text description for og:description/twitter:description, the
//! `<meta name="description">` tag and feed summaries.
//!
//! Hybrid sourcing (decided in the PR 13 review): a hand-written
//! `description` in the post front matter wins; otherwise a plain-text
//! excerpt is derived from the post body (markdown → HTML → text, capped at
//! ~160 chars which is where search engines and card UIs truncate anyway).

use askama::Values;
use tracing::error;

/// Meta descriptions get cut around 155–160 chars by search engines and
/// social cards — truncate at a word boundary before that.
const EXCERPT_MAX_LEN: usize = 160;

struct EmptyValues;

impl Values for EmptyValues {
    fn get_value<'a>(&'a self, _key: &str) -> Option<&'a dyn std::any::Any> {
        return None;
    }
}

/// Description for a post: front-matter description when present, otherwise
/// an excerpt derived from the markdown body.
pub fn post_description(description: Option<&str>, body: &str) -> String {
    match description.map(str::trim).filter(|d| !d.is_empty()) {
        Some(description) => collapse_whitespace(description),
        None => excerpt_from_body(body),
    }
}

/// First content rows of the post, rendered to plain text and capped.
fn excerpt_from_body(body: &str) -> String {
    let html = crate::filters::truncate_md::default()
        .with_rows(2)
        .execute(body, &EmptyValues)
        .and_then(|truncated| crate::filters::parse_markdown::default().execute(&truncated, &EmptyValues))
        .unwrap_or_else(|err| {
            error!("Failed to render post excerpt: {err}");
            String::new()
        });

    let text = collapse_whitespace(&strip_html(&html));
    cap_at_word(&text, EXCERPT_MAX_LEN)
}

/// Drop tags from rendered markdown, decode the entities pulldown-cmark
/// emits, and collapse runs of whitespace.
fn strip_html(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            ch if !in_tag => text.push(ch),
            _ => {}
        }
    }

    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Cut at a word boundary and add an ellipsis when anything was removed.
fn cap_at_word(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        return text.to_string();
    }

    let cut = text
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|&i| i <= max_len)
        .last()
        .unwrap_or(0);
    let cut = text[..cut]
        .rfind(' ')
        .unwrap_or(max_len.min(text.len()));
    format!("{}\u{2026}", text[..cut].trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_written_description_wins() {
        assert_eq!(
            post_description(Some("  Hand written   summary.  "), "Body"),
            "Hand written summary."
        );
    }

    #[test]
    fn blank_description_falls_back_to_excerpt() {
        let body = "First line of the post.\n\nSecond line of the post.";
        assert_eq!(
            post_description(Some("   "), body),
            "First line of the post. Second line of the post."
        );
    }

    #[test]
    fn excerpt_is_plain_text_from_markdown() {
        let body = "A **bold** start with [a link](/blog/x) and more text following it here.";
        let description = post_description(None, body);
        assert!(description.starts_with("A bold start with a link and more text"));
        assert!(!description.contains('<'));
        assert!(!description.contains('['));
    }

    #[test]
    fn excerpt_skips_headings_and_images() {
        let body = "# Heading\n![alt](/images/uploads/x.jpg)\nReal first sentence here.\nSecond sentence.";
        assert!(post_description(None, body).starts_with("Real first sentence here."));
    }

    #[test]
    fn long_excerpt_caps_at_word_boundary_with_ellipsis() {
        let body = "word ".repeat(60).trim_end().to_string();
        let description = post_description(None, &body);
        assert!(description.chars().count() <= EXCERPT_MAX_LEN + 1); // + ellipsis
        assert!(description.ends_with('\u{2026}'));
        assert!(!description.ends_with(" \u{2026}"));
    }

    #[test]
    fn entities_are_decoded() {
        let body = "Fish &amp; chips — isn't it great";
        // smart punctuation renders the apostrophe as U+2019
        assert_eq!(
            post_description(None, body),
            "Fish & chips — isn\u{2019}t it great"
        );
    }
}
