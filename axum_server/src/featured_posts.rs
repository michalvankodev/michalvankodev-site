use crate::{pages::post::PostMetadata, post_list::get_post_list, post_parser::ParseResult};
use axum::http::StatusCode;

pub async fn get_featured_posts() -> Result<Vec<ParseResult<PostMetadata>>, StatusCode> {
    let post_list = get_post_list::<PostMetadata>().await?;

    let featured_posts = post_list
        .into_iter()
        .filter(|post| post.metadata.segments.contains(&"featured".to_string()))
        .collect();

    Ok(featured_posts)
}
