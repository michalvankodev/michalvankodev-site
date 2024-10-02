use axum::{
    extract::{OriginalUri, Path},
    http::StatusCode,
};
use tokio::try_join;
use tracing::debug;

use crate::{
    blog_posts::{
        blog_post_model::{BlogPostMetadata, BLOG_POST_PATH},
        tag_list::{get_popular_tags, get_posts_by_tag},
    },
    components::site_header::{HeaderProps, Link},
    post_utils::post_listing::get_post_list,
    projects::featured_projects::get_featured_projects,
};

use super::blog_post_list::PostListTemplate;

pub async fn render_broadcast_post_list(
    tag: Option<Path<String>>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let (popular_tags, featured_projects, mut post_list) = try_join!(
        get_popular_tags(Some("broadcasts".to_string())),
        get_featured_projects(),
        get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
    )?;

    post_list.sort_by_key(|post| post.metadata.date);
    post_list.retain(|post| post.metadata.published);
    post_list.retain(|post| post.metadata.segments.contains(&"broadcasts".to_string()));
    post_list.reverse();

    let posts = get_posts_by_tag(post_list, &tag);

    let header_props = match tag {
        Some(_) => HeaderProps::with_back_link(Link {
            href: "/broadcasts".to_string(),
            label: "All broadcasts".to_string(),
        }),
        None => HeaderProps::default(),
    };

    debug!("uri:{:?}", original_uri);

    let title = if let Some(tag) = &tag {
        format!("#{tag} broadcasts")
    } else {
        "Broadcasts".to_string()
    };

    Ok(PostListTemplate {
        title: title.clone(),
        og_title: title,
        segment: "broadcasts".to_string(),
        posts,
        header_props,
        tags: popular_tags,
        featured_projects,
        current_url: original_uri.to_string(),
    })
}
