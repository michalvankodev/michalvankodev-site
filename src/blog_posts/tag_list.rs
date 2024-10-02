use axum::http::StatusCode;
use std::collections::HashMap;
use tracing::debug;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH},
    post_utils::{post_listing::get_post_list, post_parser::ParseResult},
};

pub async fn get_popular_tags(segment: Option<String>) -> Result<Vec<String>, StatusCode> {
    const TAGS_LENGTH: usize = 7;

    let mut post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;
    post_list.retain(|post| post.metadata.published);

    if let Some(segment) = segment {
        post_list.retain(|post| post.metadata.segments.contains(&segment));
    }

    let tags_sum = post_list
        .into_iter()
        .flat_map(|post| post.metadata.tags)
        .fold(HashMap::new(), |mut acc, tag| {
            *acc.entry(tag).or_insert(0) += 1;
            acc
        });

    let mut sorted_tags_by_count: Vec<_> = tags_sum.into_iter().collect();
    sorted_tags_by_count.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    // Log the counts
    for (tag, count) in &sorted_tags_by_count {
        debug!("Tag: {}, Count: {}", tag, count);
    }

    let popular_tags = sorted_tags_by_count
        .into_iter()
        .map(|tag_count| tag_count.0)
        .filter(|tag| tag != "dev")
        .take(TAGS_LENGTH)
        .collect::<Vec<String>>();

    Ok(popular_tags)
}

pub fn get_posts_by_tag(
    post_list: Vec<ParseResult<BlogPostMetadata>>,
    tag: &Option<String>,
) -> Vec<ParseResult<BlogPostMetadata>> {
    match tag {
        Some(tag) => post_list
            .into_iter()
            .filter(|post| {
                post.metadata
                    .tags
                    .iter()
                    .map(|post_tag| post_tag.to_lowercase())
                    .collect::<String>()
                    .contains(&tag.to_lowercase())
            })
            .collect(),
        None => post_list,
    }
}
