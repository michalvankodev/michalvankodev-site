use crate::{pages::post::PostMetadata, post_list::get_post_list};
use axum::http::StatusCode;
use std::collections::HashMap;
use tracing::debug;

pub async fn get_popular_blog_tags() -> Result<Vec<String>, StatusCode> {
    const TAGS_LENGTH: usize = 7;

    let post_list = get_post_list::<PostMetadata>().await?;
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

    let popular_blog_tags = sorted_tags_by_count
        .into_iter()
        .map(|tag_count| tag_count.0)
        .filter(|tag| tag != "dev")
        .take(TAGS_LENGTH)
        .collect::<Vec<String>>();

    Ok(popular_blog_tags)
}
