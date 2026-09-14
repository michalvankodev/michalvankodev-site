use askama::Values;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use lol_html::{element, errors::RewritingError, rewrite_str, RewriteStrSettings};
use rss::extension::atom::{AtomExtension, Link as AtomLink};
use rss::{Channel, ChannelBuilder, EnclosureBuilder, GuidBuilder, ItemBuilder};
use tracing::{error, warn};

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

/// Absolutize a fragment-only reference (`#heading`) against the post's
/// canonical URL. In a feed reader `href="#x"` resolves against the
/// *reader's* document (W3C validator: ContainsRelRef) and jumps nowhere
/// useful; pointing it at the post makes the link work everywhere.
fn absolutize_fragment(href: &str, post_url: &str) -> Option<String> {
    if href.starts_with('#') && href.len() > 1 {
        Some(format!("{post_url}{href}"))
    } else {
        None
    }
}

/// Absolutize every URL candidate in a `srcset` value, keeping resolution
/// descriptors (`1x`, `640w`) intact. Candidates are comma-separated.
/// Returns the input unchanged (byte-identical) when no candidate is
/// root-relative.
fn absolutize_srcset(srcset: &str) -> String {
    let needs_rewrite = srcset.split(',').any(|candidate| {
        let url = candidate.trim().split_whitespace().next().unwrap_or("");
        is_root_relative(url)
    });
    if !needs_rewrite {
        return srcset.to_string();
    }
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

/// Length of the well-formed entity reference at the start of `after`
/// (e.g. `amp;` for `&amp;x`, `#x26;` for `&#x26;x`), or 0 if `&` is a
/// stray ampersand.
fn entity_len(after: &str) -> usize {
    let bytes = after.as_bytes();
    let mut i = 0;
    if bytes.first() == Some(&b'#') {
        i = 1;
    }
    let start = i;
    while i < bytes.len() && bytes[i].is_ascii_alphanumeric() && i - start < 10 {
        i += 1;
    }
    if i > start && i < bytes.len() && bytes[i] == b';' {
        i + 1
    } else {
        0
    }
}

/// `&` that is not part of a well-formed entity reference → `&amp;`.
/// lol_html hands attribute values back in serialized form: entities that
/// were already there (`&amp;`, `&#x26;`) must survive, while a stray `&`
/// from hand-written embed HTML (raw `?video=1&parent=x`) must be escaped
/// or the feed content is invalid HTML (the validator's NotHtml).
fn escape_raw_ampersands(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 8);
    let mut rest = value;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let after = &rest[pos + 1..];
        let n = entity_len(after);
        if n > 0 {
            out.push_str(&rest[pos..pos + 1 + n]);
            rest = &rest[pos + 1 + n..];
        } else {
            out.push_str("&amp;");
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// Human label for an embed link card, derived from the embed host
/// (specs/w3c-validation.md S9). Feed readers commonly sanitize iframes
/// away, so the card tells the reader where the embed would have been.
fn embed_label(src: &str) -> &'static str {
    let host = src
        .split_once("//")
        .map(|(_, rest)| rest)
        .unwrap_or(src)
        .split(['/', '?'])
        .next()
        .unwrap_or("");
    if host.contains("twitch") {
        "watch on Twitch"
    } else if host.contains("youtube") || host.contains("youtu.be") {
        "watch on YouTube"
    } else if host.contains("spotify") {
        "listen on Spotify"
    } else if host.contains("vimeo") {
        "watch on Vimeo"
    } else {
        "open the embedded content"
    }
}

/// Make all URLs in rendered post HTML absolute. Feed readers render this
/// markup inside their own document context, so root-relative URLs would
/// resolve against the reader's origin (the RSS Best Practices Profile
/// requires absolute URLs in feed content).
///
/// `post_url` is the post's canonical absolute URL — fragment-only `#anchor`
/// references (post TOC links) are rewritten against it.
///
/// Iframes are additionally *replaced* with link cards: the W3C feed
/// validator flags any iframe in `content:encoded` as `SecurityRisk`, and
/// readers strip them anyway. The site keeps the live embed.
///
/// The rewrite is attribute-aware (real element attributes only, never text
/// content), so `href="/…"`-style snippets displayed inside code samples are
/// safe by construction. See specs/feed-image-url.md.
fn absolutize_html(html: &str, post_url: &str) -> String {
    let settings = RewriteStrSettings::new()
        .append_element_content_handler(element!("a[href]", |el| {
            if let Some(href) = el.get_attribute("href") {
                let rewritten = absolutize_url(&href)
                    .or_else(|| absolutize_fragment(&href, post_url));
                if let Some(abs) = rewritten {
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
                let absolutized = absolutize_srcset(&srcset);
                // Only touch the attribute when something actually changed —
                // already-absolute srcsets pass through byte-identical.
                if absolutized != srcset {
                    el.set_attribute("srcset", &absolutized)?;
                }
            }
            Ok(())
        }))
        // Feed embed policy (S9): iframes never ship in feed content —
        // they are a SecurityRisk per the W3C feed validator and readers
        // commonly sanitize them. Replace with a link card to the original
        // (already absolute) embed URL; the site keeps the live embed.
        .append_element_content_handler(element!("iframe", |el| {
            if el.removed() {
                return Ok(());
            }
            match el.get_attribute("src").filter(|src| !src.is_empty()) {
                Some(src) => {
                    let href = absolutize_url(&src).unwrap_or(src);
                    el.before(
                        &format!(
                            "<p><a href=\"{}\">▶ {}</a></p>",
                            escape_raw_ampersands(&href),
                            embed_label(&href)
                        ),
                        lol_html::html_content::ContentType::Html,
                    );
                    el.remove();
                }
                None => {
                    // No src — nothing to link to; drop the empty shell.
                    el.remove();
                }
            }
            Ok(())
        }))
        // Media carriers that stay elements in the feed — kept absolutized
        // so the feed-level invariant (no root-relative src anywhere) holds
        // for future content, not just today's.
        .append_element_content_handler(element!("video[src], audio[src], source[src]", |el| {
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
            let url = post_url(&segment, &post.slug);
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
            }), &url);
            let content_html = absolutize_html(&parse_markdown::default()
                .execute(&post.body, &EmptyValues)
                .unwrap_or("Can't process full post body".to_string()), &url);
            FeedItem {
                title: post.metadata.title,
                url,
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

/// Byte size of a site-local asset, for the RSS enclosure `length`
/// attribute (the RSS Best Practices Profile requires a positive integer —
/// the W3C validator errors on the empty value the builder defaults to).
/// Returns `None` for external URLs or unreadable files; the caller then
/// skips the enclosure rather than emit an invalid length.
async fn local_file_length(url: &str) -> Option<u64> {
    let path = url.strip_prefix(SITE_URL)?;
    let length = tokio::fs::metadata(format!("static{path}")).await.ok()?.len();
    (length > 0).then_some(length)
}

pub async fn render_rss_feed() -> Result<impl IntoResponse, StatusCode> {
    let feed_items = build_feed_items().await?;
    let response = build_rss_channel(&feed_items).await.to_string();
    Ok(([(header::CONTENT_TYPE, "application/xml")], response))
}

/// Assemble the RSS channel from shared feed items. Split out of the
/// handler so tests can audit the serialized feed (channel metadata,
/// enclosures, atom:link) without an HTTP round-trip.
async fn build_rss_channel(feed_items: &[FeedItem]) -> Channel {
    let last_build_date = Utc::now().to_rfc2822();
    // Channel pubDate reflects the newest content in the feed (items are
    // sorted newest-first), not the oldest.
    let publish_date = feed_items
        .first()
        .map_or_else(|| last_build_date.clone(), |item| item.date.to_rfc2822());

    let mut post_items = Vec::with_capacity(feed_items.len());
    for item in feed_items {
        let enclosure = item.image.as_ref().and_then(|url| {
            let mime_type = mime_guess::from_path(url)
                .first()
                .map(|mime| mime.to_string())
                .unwrap_or("image".to_string());
            Some((url, mime_type))
        });
        let enclosure = match enclosure {
            // The RSS Profile mandates a positive-integer length; when we
            // can't stat the file there is no honest value, so drop the
            // enclosure (the thumbnail survives in JSON Feed and og:image).
            Some((url, mime_type)) => match local_file_length(url).await {
                Some(length) => Some(
                    EnclosureBuilder::default()
                        .url(url.clone())
                        .length(length.to_string())
                        .mime_type(mime_type)
                        .build(),
                ),
                None => {
                    warn!(url, "enclosure dropped: cannot stat file for length");
                    None
                }
            },
            None => None,
        };
        post_items.push(
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
                .build(),
        );
    }

    ChannelBuilder::default()
        .title("michalvanko.dev latest posts".to_string())
        .link(SITE_URL.to_string())
        .description("Latest posts published on michalvanko.dev blog site".to_string())
        .language(Some("en".to_string()))
        .webmaster(Some("michalvankosk@gmail.com (Michal Vanko)".to_string()))
        .pub_date(Some(publish_date))
        .last_build_date(Some(last_build_date))
        .atom_ext(AtomExtension {
            links: vec![AtomLink {
                href: format!("{SITE_URL}/feed.xml"),
                rel: "self".to_string(),
                mime_type: Some("application/rss+xml".to_string()),
                ..AtomLink::default()
            }],
        })
        .items(post_items)
        .build()
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
    use std::str::FromStr;

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
            // S8: no figure/code-card nested inside <p> anywhere in the
            // corpus — the shape the W3C validator reports as NotHtml.
            assert!(
                !crate::filters::paragraph_nests_block(&item.content_html),
                "block element nested inside <p> in {}",
                item.url
            );
            // S9: iframes are replaced by link cards in feed content.
            assert!(
                !item.content_html.contains("<iframe"),
                "iframe leaked into feed content for {}",
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

    /// Collect root-relative or fragment-only URL attribute values from an
    /// HTML string — the exact properties the W3C feed validator flags
    /// (`ContainsRelRef`; fragments and empty hrefs count as relative too).
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
                } else if is_root_relative(value)
                    || (attr == "href" && (value.starts_with('#') || value.is_empty()))
                {
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
            "https://michalvanko.dev/blog/x",
        );
        assert_eq!(
            out,
            r#"<p><a href="https://michalvanko.dev/blog/x">post</a> and <img src="https://michalvanko.dev/images/uploads/pi-logo.svg" alt="logo"></p>"#
        );
    }

    #[test]
    fn fragment_hrefs_resolve_against_post_url() {
        // Post TOC links (`[text](#heading)`) render as href="#heading" —
        // on the site they jump within the page, in a reader they resolve
        // against the reader's document. They must point at the post.
        let out = absolutize_html(
            r##"<p><a href="#meet-pi">Meet Pi</a> and <a href="/blog">home</a></p>"##,
            "https://michalvanko.dev/blog/2026-04-01-week-with-my-pi-agent",
        );
        assert_eq!(
            out,
            r##"<p><a href="https://michalvanko.dev/blog/2026-04-01-week-with-my-pi-agent#meet-pi">Meet Pi</a> and <a href="https://michalvanko.dev/blog">home</a></p>"##
        );
    }

    #[test]
    fn absolutizes_srcset_candidates_and_keeps_descriptors() {
        // Shape mirrors picture_markup_generator output.
        let input = r#"<picture><source srcset="/generated_images/a_300x200.jpg 1x, /generated_images/a_600x400.jpg 2x" type="image/jpeg"><img src="/generated_images/a_600x400.jpg" alt="a"></picture>"#;
        let out = absolutize_html(input, "https://michalvanko.dev/blog/x");
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
            absolutize_html(input, "https://michalvanko.dev/blog/x"),
            input,
            "already-absolute URLs must pass through byte-identical"
        );
    }

    #[test]
    fn multi_candidate_absolute_srcset_is_byte_identical() {
        // Guards the unconditional-set_attribute regression: an
        // already-absolute multi-candidate srcset must not be re-serialized
        // (separator/descriptor normalization would change bytes).
        let input = r#"<img src="https://cdn.example.org/i.png" srcset="https://cdn.example.org/a.png 1x,https://cdn.example.org/b.png 2x">"#;
        assert_eq!(absolutize_html(input, "https://michalvanko.dev/blog/x"), input);
    }

    #[test]
    fn iframes_become_link_cards_with_host_labels() {
        // S9: the embed stays on the site, the feed gets a link card.
        // `&` in the URL must come back escaped — lol_html decodes attribute
        // values, and `before()` inserts raw HTML (NotHtml otherwise).
        let out = absolutize_html(
            r#"<iframe src="https://player.twitch.tv/?video=1427225407&amp;parent=michalvanko.dev" width="100%"></iframe>"#,
            "https://michalvanko.dev/blog/x",
        );
        assert!(!out.contains("<iframe"), "iframe must not ship in feed content: {out}");
        assert!(
            out.contains("<p><a href=\"https://player.twitch.tv/?video=1427225407&amp;parent=michalvanko.dev\">▶ watch on Twitch</a></p>"),
            "link card preserves the embed URL (escaped) and names the host: {out}"
        );
        assert!(!out.contains("&parent="), "no raw ampersand: {out}");

        // Real corpus shape: hand-written embed HTML carries a RAW `&` in the
        // src (posts write `?video=1&parent=x` unescaped). It must come out
        // escaped — without double-escaping already-escaped entities.
        let out = absolutize_html(
            r#"<iframe src="https://player.twitch.tv/?video=1728904048&parent=localhost"></iframe>"#,
            "https://michalvanko.dev/blog/x",
        );
        assert!(
            out.contains("href=\"https://player.twitch.tv/?video=1728904048&amp;parent=localhost\""),
            "raw ampersand escaped once: {out}"
        );
        assert!(!out.contains("&amp;amp;"), "no double escape: {out}");

        let out = absolutize_html(
            r#"<iframe src="https://www.youtube.com/embed/hoLMdrD5pic"></iframe>"#,
            "https://michalvanko.dev/blog/x",
        );
        assert!(out.contains("▶ watch on YouTube"), "{out}");

        let out = absolutize_html(
            r#"<iframe src="https://open.spotify.com/embed-podcast/episode/0NE6Gakn6wUP6oj7Rq4viR"></iframe>"#,
            "https://michalvanko.dev/blog/x",
        );
        assert!(out.contains("▶ listen on Spotify"), "{out}");
    }

    #[test]
    fn root_relative_iframe_src_is_absolutized_in_the_link_card() {
        let out = absolutize_html(
            r#"<iframe src="/embed/player"></iframe>"#,
            "https://michalvanko.dev/blog/x",
        );
        assert!(!out.contains("<iframe"), "{out}");
        assert!(
            out.contains("href=\"https://michalvanko.dev/embed/player\""),
            "root-relative embed src absolutized in the card: {out}"
        );
    }

    #[test]
    fn srcless_iframe_is_removed_without_a_card() {
        let out = absolutize_html(
            "<iframe width=\"560\"></iframe><p>kept</p>",
            "https://michalvanko.dev/blog/x",
        );
        assert!(!out.contains("<iframe"), "{out}");
        assert!(!out.contains("<a href"), "nothing to link to: {out}");
        assert!(out.contains("<p>kept</p>"), "sibling content intact: {out}");
    }

    #[test]
    fn code_sample_text_is_byte_identical() {
        // syntect/pulldown output escapes code-sample text (`<` → `&lt;`,
        // `"` → `&quot;`), so `href="/about"`-style snippets never exist as
        // real attributes. A naive string replace would corrupt them; the
        // attribute-aware rewrite must not change a single byte.
        let input = r#"<pre style="background-color:#2b303b;"><code>&lt;a href=&quot;/about&quot;&gt;about&lt;/a&gt; — and src=&quot;/images/x.png&quot;</code></pre>"#;
        assert_eq!(
            absolutize_html(input, "https://michalvanko.dev/blog/x"),
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
        let out = absolutize_html(&html, "https://michalvanko.dev/blog/x");
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

    /// RSS channel audit: the properties the W3C feed validator checks —
    /// atom:link rel=self, webMaster with a real name, channel pubDate =
    /// newest item, and every enclosure carrying a positive-integer length.
    #[tokio::test]
    async fn rss_channel_validates_against_w3c_findings() {
        let items = build_feed_items().await.expect("feed items built");
        let channel = build_rss_channel(&items).await;

        let atom_links = channel
            .atom_ext()
            .expect("atom extension present")
            .links();
        let self_link = atom_links
            .iter()
            .find(|l| l.rel == "self")
            .expect("atom:link rel=self present");
        assert_eq!(self_link.href, format!("{SITE_URL}/feed.xml"));
        assert_eq!(self_link.mime_type.as_deref(), Some("application/rss+xml"));

        assert_eq!(
            channel.webmaster(),
            Some("michalvankosk@gmail.com (Michal Vanko)"),
            "webMaster needs email (Real Name) per the RSS Profile"
        );

        assert!(!items.is_empty());
        let newest = items[0].date.to_rfc2822();
        assert_eq!(
            channel.pub_date(),
            Some(newest.as_str()),
            "channel pubDate must be the newest item's date (items are newest-first)"
        );

        for item in channel.items() {
            if let Some(enclosure) = item.enclosure() {
                let length: u64 = enclosure
                    .length()
                    .parse()
                    .unwrap_or_else(|_| panic!("length must be an integer: {}", enclosure.length()));
                assert!(
                    length > 0,
                    "enclosure length must be a positive integer: {}",
                    enclosure.url()
                );
            }
        }

        // Round-trip through the serializer: the reader must parse what we
        // emit (also exercises xmlns:atom handling for the self link).
        let xml = channel.to_string();
        let reparsed = rss::Channel::from_str(&xml).expect("serialized feed re-parses");
        assert!(reparsed.atom_ext().is_some());
    }
}
