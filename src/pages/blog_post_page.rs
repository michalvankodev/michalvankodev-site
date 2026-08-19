use askama::Template;
use axum::extract::OriginalUri;
use axum::response::{Html, IntoResponse};
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};

use crate::blog_posts::blog_post_model::{Segment, BLOG_POST_PATH};
use crate::filters::HeadingToc;
use crate::post_utils::post_listing::get_post_list;
use crate::post_utils::post_parser::ParseResult;
use crate::{
    blog_posts::blog_post_model::BlogPostMetadata, components::site_header::Link, filters,
    post_utils::post_parser::parse_post,
};

use crate::components::site_header::HeaderProps;

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate {
    pub title: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub segment: Segment,
    pub header_props: HeaderProps,
    pub slug: String,
    pub thumbnail: Option<String>,
    pub toc: Vec<HeadingToc>,
    pub reading_time: u32,
    pub newer_post: Option<ParseResult<BlogPostMetadata>>,
    pub older_post: Option<ParseResult<BlogPostMetadata>>,
    pub recommended_posts: Vec<ParseResult<BlogPostMetadata>>,
}

pub async fn render_blog_post(
    Path(post_id): Path<String>,
    OriginalUri(original_uri): OriginalUri,
) -> Result<impl IntoResponse, StatusCode> {
    let path = format!("{}/{}.md", BLOG_POST_PATH, post_id);
    let post = parse_post::<BlogPostMetadata>(&path).await?;
    let segment = if original_uri.to_string().starts_with("/blog") {
        Segment::Blog
    } else if original_uri.to_string().starts_with("/broadcasts") {
        Segment::Broadcasts
    } else {
        Segment::Blog
    };

    let mut recommended_posts = get_recommended_posts(&segment, &post.metadata.tags).await?;
    recommended_posts.retain(|post| post.slug != post_id);
    recommended_posts.sort_by_key(|post| post.slug.to_string());
    recommended_posts.reverse();
    let recommended_posts = recommended_posts.into_iter().take(2).collect::<Vec<_>>();

    let toc = filters::extract_headings(&post.body);
    let reading_time = (post.body.split_whitespace().count() as u32 / 220).max(1);
    let (newer_post, older_post) = get_post_neighbors(&segment, &post_id).await?;

    let header_props = match segment {
        Segment::Blog => HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
        Segment::Broadcasts => HeaderProps::with_back_link(Link {
            href: "/broadcasts".to_string(),
            label: "All broadcasts".to_string(),
        }),
        _ => HeaderProps::default(),
    };

    Ok(Html(
        BlogPostTemplate {
            title: post.metadata.title,
            date: post.metadata.date,
            tags: post.metadata.tags,
            body: post.body,
            slug: post.slug,
            segment,
            thumbnail: post.metadata.thumbnail,
            toc,
            reading_time,
            newer_post,
            older_post,
            header_props,
            recommended_posts,
        }
        .render()
        .unwrap(),
    ))
}

/// Return the (newer, older) neighbors of a post within its segment,
/// ordered by date descending.
async fn get_post_neighbors(
    segment: &Segment,
    current_slug: &str,
) -> Result<
    (
        Option<ParseResult<BlogPostMetadata>>,
        Option<ParseResult<BlogPostMetadata>>,
    ),
    StatusCode,
> {
    let posts = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;
    let mut posts: Vec<_> = posts
        .into_iter()
        .filter(|post| {
            post.metadata
                .segments
                .iter()
                .any(|post_segment| post_segment == segment)
        })
        .collect();
    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    let index = posts.iter().position(|post| post.slug == current_slug);
    let mut posts = posts.into_iter();
    let (newer, older) = match index {
        Some(i) => {
            let newer = if i > 0 { posts.nth(i - 1) } else { None };
            // after `newer` was taken, the iterator sits on the current post;
            // skipping it yields the next older one
            let older = posts.nth(1);
            (newer, older)
        }
        None => (None, None),
    };
    Ok((newer, older))
}

async fn get_recommended_posts(
    segment: &Segment,
    tags: &[String],
) -> Result<Vec<ParseResult<BlogPostMetadata>>, StatusCode> {
    let posts = get_post_list::<BlogPostMetadata>(BLOG_POST_PATH).await?;

    let recommended_posts = posts
        .into_iter()
        .filter(|post| {
            let is_same_segment = post
                .metadata
                .segments
                .iter()
                .any(|post_segment| post_segment == segment);
            let has_same_tags = post
                .metadata
                .tags
                .iter()
                .any(|post_tag| tags.contains(post_tag));
            is_same_segment && has_same_tags
        })
        .collect::<Vec<_>>();

    Ok(recommended_posts)
}
