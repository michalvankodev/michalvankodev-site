use askama::Template;
use axum::{
    extract::{OriginalUri, Path, RawQuery},
    http::StatusCode,
};
use tokio::try_join;
use tracing::debug;

use crate::{
    blog_posts::{
        blog_post_model::{BlogPostMetadata, BLOG_POST_PATH},
        tag_list::get_popular_blog_tags,
    },
    components::site_header::{HeaderProps, Link},
    filters,
    post_utils::{post_listing::get_post_list, post_parser::ParseResult},
    projects::{featured_projects::get_featured_projects, project_model::ProjectMetadata},
};

#[derive(Template)]
#[template(path = "blog_post_list.html")]
pub struct PostListTemplate {
    pub title: String,
    pub posts: Vec<ParseResult<BlogPostMetadata>>,
    pub tag: Option<String>,
    pub header_props: HeaderProps,
    pub blog_tags: Vec<String>,
    pub featured_projects: Vec<ParseResult<ProjectMetadata>>,
    pub current_url: String,
}

pub async fn render_blog_post_list(
    tag: Option<Path<String>>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let (blog_tags, featured_projects, mut post_list) = try_join!(
        get_popular_blog_tags(),
        get_featured_projects(),
        get_post_list::<BlogPostMetadata>(BLOG_POST_PATH)
    )?;

    post_list.sort_by_key(|post| post.metadata.date);
    post_list.retain(|post| post.metadata.published);
    post_list.reverse();

    let posts = match &tag {
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
    };

    let header_props = match tag {
        Some(_) => HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
        None => HeaderProps::default(),
    };

    debug!("uri:{:?}", original_uri);

    let title = if let Some(tag) = &tag {
        format!("{tag} blog posts")
    } else {
        "Blog posts".to_string()
    };

    Ok(PostListTemplate {
        title,
        posts,
        tag,
        header_props,
        blog_tags,
        featured_projects,
        current_url: original_uri.to_string(),
    })
}
