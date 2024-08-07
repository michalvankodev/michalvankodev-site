use askama::Template;
use axum::{extract::Path, http::StatusCode};

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, BLOG_POST_PATH},
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::{HeaderProps, Link},
    },
    filters,
    post_utils::{post_listing::get_post_list, post_parser::ParseResult},
};

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub title: String,
    pub posts: Vec<ParseResult<BlogPostMetadata>>,
    pub tag: Option<String>,
    pub site_footer: SiteFooter,
    pub header_props: HeaderProps,
}

pub async fn render_blog_post_list(
    tag: Option<Path<String>>,
) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let site_footer = render_site_footer().await?;
    let mut post_list = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;
    post_list.sort_by_key(|post| post.metadata.date);
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

    Ok(PostListTemplate {
        title: "Posts".to_owned(),
        posts,
        tag,
        site_footer,
        header_props,
    })
}
