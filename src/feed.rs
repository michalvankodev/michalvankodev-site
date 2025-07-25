use askama::Values;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use chrono::Utc;
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, Item, ItemBuilder};

use crate::blog_posts::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH};
use crate::filters::{parse_markdown, truncate_md};
use crate::post_utils::post_listing::get_post_list;

struct EmptyValues;

impl Values for EmptyValues {
    fn get_value<'a>(&'a self, _key: &str) -> Option<&'a dyn std::any::Any> {
        return None;
    }
}

pub async fn render_rss_feed() -> Result<impl IntoResponse, StatusCode> {
    let mut post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
        .await
        .unwrap_or(vec![]);
    post_list.sort_by_key(|post| post.metadata.date);
    post_list.reverse();

    let last_build_date = Utc::now().to_rfc2822();
    let publish_date = post_list.last().map_or_else(
        || last_build_date.clone(),
        |post| post.metadata.date.to_rfc2822(),
    );

    let post_items = post_list
        .into_iter()
        .map(|post| {
            ItemBuilder::default()
                .title(Some(post.metadata.title))
                .link(Some(format!("https://michalvanko.dev/blog/{}", post.slug)))
                .description({
                    let truncated = truncate_md(&post.body, &EmptyValues, 2)
                        .unwrap_or("Can't parse post body".to_string());
                    let parsed_md = parse_markdown(&truncated, &EmptyValues)
                        .unwrap_or("Can't process truncated post body".to_string());
                    Some(parsed_md)
                })
                .content({
                    let parsed_md = parse_markdown(&post.body, &EmptyValues)
                        .unwrap_or("Can't process full post body".to_string());
                    Some(parsed_md)
                })
                .enclosure({
                    post.metadata.thumbnail.map(|src| {
                        let mime_type = mime_guess::from_path(&src)
                            .first()
                            .map(|mime| mime.to_string())
                            .unwrap_or("image".to_string());
                        EnclosureBuilder::default()
                            .url(src)
                            .mime_type(mime_type)
                            .build()
                    })
                })
                .guid(Some(
                    GuidBuilder::default()
                        .value(format!("https://michalvanko.dev/blog/{}", post.slug))
                        .build(),
                ))
                .pub_date(Some(post.metadata.date.to_rfc2822()))
                .build()
        })
        .collect::<Vec<Item>>();

    let feed_builder = ChannelBuilder::default()
        .title("michalvanko.dev latest posts".to_string())
        .link("https://michalvanko.dev".to_string())
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
