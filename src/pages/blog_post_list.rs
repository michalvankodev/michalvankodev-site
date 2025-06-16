use askama::Template;
use axum::{
    extract::{OriginalUri, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tokio::try_join;
use tracing::debug;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH},
    components::site_header::{HeaderProps, Link},
    post_utils::{
        post_listing::get_post_list,
        segments::get_posts_by_segment,
        tags::{get_popular_tags, get_posts_by_tag},
    },
    projects::featured_projects::get_featured_projects,
};

use super::post_list::PostListTemplate;

// TODO Refactor to render post list for the same broadcasts, blog and cookbook
pub async fn render_blog_post_list(
    tag: Option<Path<String>>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<impl IntoResponse, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let (blog_tags, featured_projects, post_list) = try_join!(
        get_popular_tags(Some(Segment::Blog)),
        get_featured_projects(),
        get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
    )?;

    let posts = get_posts_by_segment(post_list, &[Segment::Blog]);
    let posts = get_posts_by_tag(posts, &tag);
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

    Ok(Html(
        PostListTemplate {
            title,
            og_title,
            segment: Segment::Blog,
            posts,
            header_props,
            tags: blog_tags,
            featured_projects,
            current_url: original_uri.to_string(),
        }
        .render()
        .unwrap(),
    ))
}
