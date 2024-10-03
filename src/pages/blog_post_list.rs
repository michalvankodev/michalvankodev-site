use axum::{
    extract::{OriginalUri, Path},
    http::StatusCode,
};
use tokio::try_join;
use tracing::debug;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH},
    components::site_header::{HeaderProps, Link},
    post_utils::{
        post_listing::get_post_list,
        tags::{get_popular_tags, get_posts_by_tag},
    },
    projects::featured_projects::get_featured_projects,
};

use super::post_list::PostListTemplate;

pub async fn render_blog_post_list(
    tag: Option<Path<String>>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let (blog_tags, featured_projects, mut post_list) = try_join!(
        get_popular_tags(Some("blog".to_string())),
        get_featured_projects(),
        get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
    )?;

    post_list.sort_by_key(|post| post.metadata.date);
    post_list.retain(|post| post.metadata.published);
    post_list.retain(|post| post.metadata.segments.contains(&"blog".to_string()));
    post_list.reverse();

    let posts = get_posts_by_tag(post_list, &tag);
    let header_props = match tag {
        Some(_) => HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
        None => HeaderProps::default(),
    };

    debug!("uri:{:?}", original_uri);

    let (title, og_title) = if let Some(tag) = &tag {
        (format!("#{tag}"), format!("{tag} blog posts"))
    } else {
        ("Blog posts".to_string(), "Blog posts".to_string())
    };

    Ok(PostListTemplate {
        title,
        og_title,
        segment: "blog".to_string(),
        posts,
        header_props,
        tags: blog_tags,
        featured_projects,
        current_url: original_uri.to_string(),
    })
}
