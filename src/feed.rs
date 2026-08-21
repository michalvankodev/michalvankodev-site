use askama::Values;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, Item, ItemBuilder};

use crate::blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH};
use crate::filters::{parse_markdown, truncate_md};
use crate::post_utils::post_listing::get_post_list;
use crate::post_utils::post_parser::ParseResult;

struct EmptyValues;

impl Values for EmptyValues {
    fn get_value<'a>(&'a self, _key: &str) -> Option<&'a dyn std::any::Any> {
        return None;
    }
}

const SITE_URL: &str = "https://michalvanko.dev";

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
            let description_html = truncate_md::default()
                .with_rows(2)
                .execute(&post.body, &EmptyValues)
                .and_then(|truncated| {
                    parse_markdown::default().execute(&truncated, &EmptyValues)
                })
                .unwrap_or("Can't parse post body".to_string());
            let content_html = parse_markdown::default()
                .execute(&post.body, &EmptyValues)
                .unwrap_or("Can't process full post body".to_string());
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
}
