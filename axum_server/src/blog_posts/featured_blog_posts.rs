use axum::http::StatusCode;

use crate::post_utils::{post_listing::get_post_list, post_parser::ParseResult};

use super::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH};

pub async fn get_featured_blog_posts() -> Result<Vec<ParseResult<BlogPostMetadata>>, StatusCode> {
    let mut post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;
    post_list.retain(|post| post.metadata.segments.contains(&"featured".to_string()));
    post_list.retain(|post| post.metadata.published);
    post_list.sort_by_key(|post| post.metadata.date);
    post_list.reverse();

    Ok(post_list)
}
