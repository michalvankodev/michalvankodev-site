use axum::http::StatusCode;

use crate::post_utils::{post_listing::get_post_list, post_parser::ParseResult};

use super::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH};

pub async fn get_featured_blog_posts() -> Result<Vec<ParseResult<BlogPostMetadata>>, StatusCode> {
    let post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;

    let featured_posts = post_list
        .into_iter()
        .filter(|post| post.metadata.segments.contains(&"featured".to_string()))
        .collect();

    Ok(featured_posts)
}
