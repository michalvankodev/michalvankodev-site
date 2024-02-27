use askama::Template;
use axum::{extract::Path, http::StatusCode};

use crate::{
    components::{
        site_footer::{render_site_footer, SiteFooter},
        site_header::{HeaderProps, Link},
    },
    post_list::get_post_list,
    post_parser::ParseResult,
};

use super::post::PostMetadata;

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub title: String,
    pub posts: Vec<ParseResult<PostMetadata>>,
    pub tag: Option<String>,
    pub site_footer: SiteFooter,
    pub header_props: HeaderProps,
}

pub async fn render_post_list(tag: Option<Path<String>>) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let site_footer = tokio::spawn(render_site_footer());
    let mut post_list = get_post_list::<PostMetadata>().await?;
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

    let site_footer = site_footer
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO if we have a tag we want to go back to all posts, otherwise we don't
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

// TODO Do we want pagination or not? Ask designer/ We don't want itt
// TODO when tags are true render different "see all post" message
