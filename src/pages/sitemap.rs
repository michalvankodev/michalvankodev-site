use std::fmt::Write as _;

use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use crate::blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH};
use crate::post_utils::post_listing::get_post_list;

const SITE_URL: &str = "https://michalvanko.dev";

/// sitemap.xml generated from the post lists at render/export time.
pub async fn render_sitemap() -> Result<impl IntoResponse, StatusCode> {
    let mut urls = vec![
        (format!("{SITE_URL}/"), None),
        (format!("{SITE_URL}/blog"), None),
        (format!("{SITE_URL}/broadcasts"), None),
        (format!("{SITE_URL}/showcase"), None),
        (format!("{SITE_URL}/portfolio"), None),
        (format!("{SITE_URL}/contact"), None),
    ];

    let post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
        .await
        .unwrap_or(vec![]);
    for post in post_list {
        let segment = post
            .metadata
            .segments
            .iter()
            .find(|segment| matches!(segment, Segment::Blog | Segment::Broadcasts))
            .cloned()
            .unwrap_or(Segment::Blog);
        let loc = match segment {
            Segment::Broadcasts => format!("{SITE_URL}/broadcasts/{}", post.slug),
            _ => format!("{SITE_URL}/blog/{}", post.slug),
        };
        urls.push((loc, Some(post.metadata.date.to_rfc3339())));
    }

    let mut xml = String::new();
    writeln!(
        xml,
        r#"<?xml version="1.0" encoding="UTF-8"?>"#
    )
    .expect("infallible");
    writeln!(
        xml,
        r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#
    )
    .expect("infallible");
    for (loc, lastmod) in urls {
        match lastmod {
            Some(lastmod) => writeln!(
                xml,
                "<url><loc>{loc}</loc><lastmod>{lastmod}</lastmod></url>"
            )
            .expect("infallible"),
            None => writeln!(xml, "<url><loc>{loc}</loc></url>").expect("infallible"),
        }
    }
    writeln!(xml, "</urlset>").expect("infallible");

    Ok(([(header::CONTENT_TYPE, "application/xml")], xml))
}
