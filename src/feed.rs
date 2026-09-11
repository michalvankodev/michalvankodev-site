use askama::Values;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use lol_html::{element, errors::RewritingError, rewrite_str, RewriteStrSettings};
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, Item, ItemBuilder};
use tracing::error;

use crate::blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH};
use crate::filters::{parse_markdown, truncate_md};
use crate::post_utils::post_listing::get_post_list;

struct EmptyValues;

impl Values for EmptyValues {
    fn get_value<'a>(&'a self, _key: &str) -> Option<&'a dyn std::any::Any> {
        return None;
    }
}

const SITE_URL: &str = "https://michalvanko.dev";

/// Root-relative URL (`/…`) — the only shape that breaks in feed readers.
/// Protocol-relative `//…` and everything else is left alone.
fn is_root_relative(url: &str) -> bool {
    url.starts_with('/') && !url.starts_with("//")
}

/// Prefix a root-relative URL with the site origin. Absolute URLs and
/// protocol-relative `//…` URLs pass through untouched.
fn absolutize_url(url: &str) -> Option<String> {
    if is_root_relative(url) {
        Some(format!("{SITE_URL}{url}"))
    } else {
        None
    }
}

/// Absolutize every URL candidate in a `srcset` value, keeping resolution
/// descriptors (`1x`, `640w`) intact. Candidates are comma-separated.
fn absolutize_srcset(srcset: &str) -> String {
    srcset
        .split(',')
        .map(|candidate| {
            let candidate = candidate.trim();
            let mut parts = candidate.splitn(2, char::is_whitespace);
            let url = parts.next().unwrap_or("");
            match (absolutize_url(url), parts.next()) {
                (Some(abs), Some(descriptor)) => format!("{abs} {}", descriptor.trim_start()),
                (Some(abs), None) => abs,
                (None, _) => candidate.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Make all URLs in rendered post HTML absolute. Feed readers render this
/// markup inside their own document context, so root-relative URLs would
/// resolve against the reader's origin (JSON Feed 1.1 and the RSS Best
/// Practices Profile both require absolute URLs in feed content).
///
/// The rewrite is attribute-aware (real element attributes only, never text
/// content), so `href="/…"`-style snippets displayed inside code samples are
/// safe by construction. See specs/feed-image-url.md.
fn absolutize_html(html: &str) -> String {
    let settings = RewriteStrSettings::new()
        .append_element_content_handler(element!("a[href]", |el| {
            if let Some(href) = el.get_attribute("href") {
                if let Some(abs) = absolutize_url(&href) {
                    el.set_attribute("href", &abs)?;
                }
            }
            Ok(())
        }))
        .append_element_content_handler(element!("img[src]", |el| {
            if let Some(src) = el.get_attribute("src") {
                if let Some(abs) = absolutize_url(&src) {
                    el.set_attribute("src", &abs)?;
                }
            }
            Ok(())
        }))
        .append_element_content_handler(element!("img[srcset], source[srcset]", |el| {
            if let Some(srcset) = el.get_attribute("srcset") {
                el.set_attribute("srcset", &absolutize_srcset(&srcset))?;
            }
            Ok(())
        }))
        // Media/embed carriers posts rarely use — kept covered so the
        // feed-level invariant (no root-relative src/href/srcset anywhere)
        // holds for future content, not just today's.
        .append_element_content_handler(element!("iframe[src], video[src], audio[src], source[src]", |el| {
            if let Some(src) = el.get_attribute("src") {
                if let Some(abs) = absolutize_url(&src) {
                    el.set_attribute("src", &abs)?;
                }
            }
            Ok(())
        }));
    rewrite_str(html, settings)
    .unwrap_or_else(|e: RewritingError| {
        // Serve the un-rewritten body rather than failing the whole feed;
        // a rewriting error here would be an lol_html internal fault.
        error!(error = %e, "feed HTML absolutization failed");
        html.to_string()
    })
}

/// One feed entry, shared by RSS and JSON feed renderers.
pub struct FeedItem {
    pub title: String,
    /// Absolute URL of the post (segment-aware).
    pub url: String,
    /// Truncated excerpt as HTML.
    pub description_html: String,
    /// Full post body as HTML.
    pub content_html: String,
    pub date: DateTime<Utc>,
    /// Absolute cover image URL.
    pub image: Option<String>,
    pub tags: Vec<String>,
}

/// Absolute site URL for a post (blog posts under /blog, broadcasts under
/// /broadcasts).
fn post_url(segment: &Segment, slug: &str) -> String {
    match segment {
        Segment::Broadcasts => format!("{SITE_URL}/broadcasts/{slug}"),
        _ => format!("{SITE_URL}/blog/{slug}"),
    }
}

/// Build feed items from all published posts, newest first. Shared by the
/// RSS and JSON feed renderers so both stay in sync.
pub async fn build_feed_items() -> Result<Vec<FeedItem>, StatusCode> {
    let mut post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
        .await
        .unwrap_or(vec![]);
    post_list.sort_by_key(|post| post.metadata.date);
    post_list.reverse();

    let items = post_list
        .into_iter()
        .map(|post| {
            let segment = post
                .metadata
                .segments
                .iter()
                .find(|segment| matches!(segment, Segment::Blog | Segment::Broadcasts))
                .cloned()
                .unwrap_or(Segment::Blog);
            // Syndicated HTML must carry absolute URLs — feed readers render
            // it against their own origin. Single choke point for both
            // description and full content (see specs/feed-image-url.md).
            let description_html = absolutize_html(&post.metadata.description.clone().unwrap_or_else(|| {
                truncate_md::default()
                    .with_rows(2)
                    .execute(&post.body, &EmptyValues)
                    .and_then(|truncated| {
                        parse_markdown::default().execute(&truncated, &EmptyValues)
                    })
                    .unwrap_or("Can't parse post body".to_string())
            }));
            let content_html = absolutize_html(&parse_markdown::default()
                .execute(&post.body, &EmptyValues)
                .unwrap_or("Can't process full post body".to_string()));
            FeedItem {
                title: post.metadata.title,
                url: post_url(&segment, &post.slug),
                description_html,
                content_html,
                date: post.metadata.date,
                image: post.metadata.thumbnail.map(|src| {
                    if src.starts_with("http") {
                        src
                    } else {
                        format!("{SITE_URL}{src}")
                    }
                }),
                tags: post.metadata.tags,
            }
        })
        .collect();
    Ok(items)
}

pub async fn render_rss_feed() -> Result<impl IntoResponse, StatusCode> {
    let feed_items = build_feed_items().await?;

    let last_build_date = Utc::now().to_rfc2822();
    let publish_date = feed_items
        .last()
        .map_or_else(|| last_build_date.clone(), |item| item.date.to_rfc2822());

    let post_items = feed_items
        .iter()
        .map(|item| {
            let enclosure = item.image.as_ref().map(|url| {
                let mime_type = mime_guess::from_path(url)
                    .first()
                    .map(|mime| mime.to_string())
                    .unwrap_or("image".to_string());
                EnclosureBuilder::default()
                    .url(url.clone())
                    .mime_type(mime_type)
                    .build()
            });
            ItemBuilder::default()
                .title(Some(item.title.clone()))
                .link(Some(item.url.clone()))
                .description(Some(item.description_html.clone()))
                .content(Some(item.content_html.clone()))
                .enclosure(enclosure)
                .guid(Some(
                    GuidBuilder::default().value(item.url.clone()).build(),
                ))
                .pub_date(Some(item.date.to_rfc2822()))
                .build()
        })
        .collect::<Vec<Item>>();

    let feed_builder = ChannelBuilder::default()
        .title("michalvanko.dev latest posts".to_string())
        .link(SITE_URL.to_string())
        .description("Latest posts published on michalvanko.dev blog site".to_string())
        .language(Some("en".to_string()))
        .webmaster(Some("michalvankosk@gmail.com".to_string()))
        .pub_date(Some(publish_date))
        .last_build_date(Some(last_build_date))
        .items(post_items)
        .build();

    let response = feed_builder.to_string();
    Ok(([(header::CONTENT_TYPE, "application/xml")], response))
}

pub async fn render_json_feed() -> Result<impl IntoResponse, StatusCode> {
    let feed_items = build_feed_items().await?;
    let body = build_json_feed_body(&feed_items);
    Ok(([(header::CONTENT_TYPE, "application/feed+json")], body))
}

fn build_json_feed_body(feed_items: &[FeedItem]) -> String {
    let items = feed_items
        .iter()
        .map(|item| {
            let mut entry = serde_json::json!({
                "id": item.url,
                "url": item.url,
                "title": item.title,
                "content_html": item.content_html,
                "summary": item.description_html,
                "date_published": item.date.to_rfc3339(),
            });
            if let Some(image) = &item.image {
                entry["image"] = serde_json::json!(image);
            }
            if !item.tags.is_empty() {
                entry["tags"] = serde_json::json!(item.tags);
            }
            entry
        })
        .collect::<Vec<_>>();

    let feed = serde_json::json!({
        "version": "https://jsonfeed.org/version/1.1",
        "title": "michalvanko.dev latest posts",
        "home_page_url": SITE_URL,
        "feed_url": format!("{SITE_URL}/feed.json"),
        "icon": format!("{SITE_URL}/images/m-logo.svg"),
        "favicon": format!("{SITE_URL}/images/m-logo.svg"),
        "description": "Latest posts published on michalvanko.dev blog site",
        "language": "en",
        "items": items,
    });

    feed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RSS full-text audit: every feed item carries full parsed body HTML and
    /// absolute URLs only (links, guids, enclosures).
    #[tokio::test]
    async fn feed_items_have_full_text_and_absolute_urls() {
        let items = build_feed_items().await.expect("feed items built");
        assert!(
            !items.is_empty(),
            "feed must contain posts (_posts/blog is part of the repo)"
        );

        for item in &items {
            assert!(
                item.url.starts_with("https://michalvanko.dev/"),
                "post URL must be absolute: {}",
                item.url
            );
            assert!(
                item.content_html.contains("<p>"),
                "full content must be parsed markdown HTML for {}",
                item.url
            );
            assert!(
                !item.content_html.trim().is_empty(),
                "content body must not be empty for {}",
                item.url
            );
            if let Some(image) = &item.image {
                assert!(
                    image.starts_with("https://michalvanko.dev/") || image.starts_with("http"),
                    "enclosure/image URL must be absolute: {}",
                    image
                );
            }
        }
    }

    #[tokio::test]
    async fn json_feed_renders_with_full_content() {
        let items = build_feed_items().await.expect("feed items built");
        let body = build_json_feed_body(&items);
        let parsed: serde_json::Value =
            serde_json::from_str(&body).expect("json feed body is valid JSON");
        assert_eq!(
            parsed["version"],
            "https://jsonfeed.org/version/1.1",
            "JSON Feed version header"
        );
        assert_eq!(
            parsed["icon"],
            "https://michalvanko.dev/images/m-logo.svg",
            "feed icon points at the site logo"
        );
        let feed_items = parsed["items"].as_array().expect("items array");
        assert!(!feed_items.is_empty());
        for item in feed_items {
            let url = item["url"].as_str().expect("url");
            assert!(url.starts_with("https://michalvanko.dev/"));
            assert!(item["content_html"]
                .as_str()
                .expect("content_html")
                .contains("<p>"));
        }
    }

    // ---------- absolutization (specs/feed-image-url.md test plan) ----------

    /// Collect root-relative URL attribute values from an HTML string — the
    /// exact property the W3C feed validator flags (`ContainsRelRef`).
    /// Text content is not scanned: code samples legitimately contain
    /// `href="/…"`-shaped *text* (inline code spans keep raw quotes), which
    /// is not an attribute — so `<pre>`/`<code>` contents are stripped first,
    /// mirroring what a real HTML parser sees.
    fn root_relative_urls(html: &str) -> Vec<String> {
        let html = strip_code_content(html);
        let mut rest = html.as_str();
        let mut hits = Vec::new();
        for attr in ["src", "href", "srcset"] {
            let needle = format!("{attr}=\"");
            while let Some(pos) = rest.find(&needle) {
                let after = &rest[pos + needle.len()..];
                let value = after.split('"').next().unwrap_or("");
                if attr == "srcset" {
                    for candidate in value.split(',') {
                        let url = candidate.trim().split_whitespace().next().unwrap_or("");
                        if is_root_relative(url) {
                            hits.push(format!("{attr}: {url}"));
                        }
                    }
                } else if is_root_relative(value) {
                    hits.push(format!("{attr}: {value}"));
                }
                rest = after;
            }
        }
        hits
    }

    /// Remove the inner content of `<pre>`/`<code>` elements so code-sample
    /// text (which may look like `attr="value"`) is not mistaken for real
    /// attributes by the naive scanner above.
    fn strip_code_content(html: &str) -> String {
        let mut out = html.to_string();
        for tag in ["pre", "code"] {
            let open_pat = format!("<{tag}");
            let close_pat = format!("</{tag}>");
            let mut search_from = 0;
            while let Some(rel) = out[search_from..].find(&open_pat) {
                let open = search_from + rel;
                let Some(gt) = out[open..].find('>') else { break };
                let content_start = open + gt + 1;
                let end = out[content_start..]
                    .find(&close_pat)
                    .map_or(out.len(), |rel| content_start + rel);
                out.replace_range(content_start..end, "");
                search_from = content_start;
            }
        }
        out
    }

    #[test]
    fn absolutizes_root_relative_href_and_src() {
        let out = absolutize_html(
            r#"<p><a href="/blog/x">post</a> and <img src="/images/uploads/pi-logo.svg" alt="logo"></p>"#,
        );
        assert_eq!(
            out,
            r#"<p><a href="https://michalvanko.dev/blog/x">post</a> and <img src="https://michalvanko.dev/images/uploads/pi-logo.svg" alt="logo"></p>"#
        );
    }

    #[test]
    fn absolutizes_srcset_candidates_and_keeps_descriptors() {
        // Shape mirrors picture_markup_generator output.
        let input = r#"<picture><source srcset="/generated_images/a_300x200.jpg 1x, /generated_images/a_600x400.jpg 2x" type="image/jpeg"><img src="/generated_images/a_600x400.jpg" alt="a"></picture>"#;
        let out = absolutize_html(input);
        assert!(
            out.contains("srcset=\"https://michalvanko.dev/generated_images/a_300x200.jpg 1x, https://michalvanko.dev/generated_images/a_600x400.jpg 2x\""),
            "srcset candidates absolutized, descriptors intact: {out}"
        );
        assert!(
            out.contains("src=\"https://michalvanko.dev/generated_images/a_600x400.jpg\""),
            "img fallback src absolutized: {out}"
        );
        assert!(root_relative_urls(&out).is_empty());
    }

    #[test]
    fn absolute_and_protocol_relative_urls_pass_through_byte_identical() {
        let input = r#"<p>See <a href="https://example.com/x?y=1">this</a> and <img src="https://cdn.example.org/i.png" srcset="https://cdn.example.org/i@2x.png 2x"> plus <a href="//other.example/y">protocol-relative</a>.</p>"#;
        assert_eq!(
            absolutize_html(input),
            input,
            "already-absolute URLs must pass through byte-identical"
        );
    }

    #[test]
    fn code_sample_text_is_byte_identical() {
        // syntect/pulldown output escapes code-sample text (`<` → `&lt;`,
        // `"` → `&quot;`), so `href="/about"`-style snippets never exist as
        // real attributes. A naive string replace would corrupt them; the
        // attribute-aware rewrite must not change a single byte.
        let input = r#"<pre style="background-color:#2b303b;"><code>&lt;a href=&quot;/about&quot;&gt;about&lt;/a&gt; — and src=&quot;/images/x.png&quot;</code></pre>"#;
        assert_eq!(
            absolutize_html(input),
            input,
            "code samples must survive the rewrite untouched"
        );
    }

    #[test]
    fn rewrites_real_attributes_but_not_code_spans() {
        let md = "Use `<a href=\"/about\">` inline, then:\n\n```html\n<a href=\"/about\">about</a>\n```\n\nAnd a [real link](/about).";
        let html = parse_markdown::default()
            .execute(md, &EmptyValues)
            .expect("markdown parses");
        let out = absolutize_html(&html);
        assert_eq!(
            root_relative_urls(&out),
            Vec::<String>::new(),
            "no root-relative attribute may survive: {out}"
        );
        assert_eq!(
            out.matches("https://michalvanko.dev/about").count(),
            1,
            "exactly one real anchor rewritten; code-span text untouched: {out}"
        );
    }

    #[tokio::test]
    async fn feed_items_carry_absolute_urls_only() {
        let items = build_feed_items().await.expect("feed items built");
        assert!(!items.is_empty());
        for item in &items {
            for field in [&item.description_html, &item.content_html] {
                let hits = root_relative_urls(field);
                assert!(
                    hits.is_empty(),
                    "root-relative URLs leaked into {}: {hits:?}",
                    item.url
                );
            }
        }
    }

    #[tokio::test]
    async fn json_feed_body_carries_absolute_urls_only() {
        let items = build_feed_items().await.expect("feed items built");
        let body = build_json_feed_body(&items);
        let parsed: serde_json::Value =
            serde_json::from_str(&body).expect("json feed body is valid JSON");
        for item in parsed["items"].as_array().expect("items array") {
            for field in ["content_html", "summary"] {
                if let Some(html) = item[field].as_str() {
                    let hits = root_relative_urls(html);
                    assert!(
                        hits.is_empty(),
                        "root-relative URLs leaked into {} ({field}): {hits:?}",
                        item["url"]
                    );
                }
            }
        }
    }
}
