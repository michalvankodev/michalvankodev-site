use askama::Template;
use axum::{extract::Path, http::StatusCode};
use tokio::fs::read_dir;
use tracing::info;

use crate::post_parser::{parse_post, ParseResult};

use super::post::PostMetadata;

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub posts: Vec<ParseResult<PostMetadata>>,
    pub tag: Option<String>,
}

pub async fn render_post_list(tag: Option<Path<String>>) -> Result<PostListTemplate, StatusCode> {
    // I will forget what happens here in a week. But essentially it's pattern matching and shadowing
    let tag = tag.map(|Path(tag)| tag);

    let path = "../_posts/blog/";
    let mut dir = read_dir(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut posts: Vec<ParseResult<PostMetadata>> = Vec::new();

    while let Some(file) = dir
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let file_path = file.path();
        let file_path_str = file_path.to_str().unwrap();
        info!(":{}", file_path_str);
        let post = parse_post::<PostMetadata>(file_path_str).await?;
        posts.push(post);
    }

    let posts = match &tag {
        Some(tag) => posts
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
        None => posts,
    };

    Ok(PostListTemplate { posts, tag })
}

// TODO Do we want pagination or not? Ask designer
// TODO How are we going to implement tags? The path extractor would have to make decision on wether we have a path or a blog post
// TODO Refactor `?` with `.map_err`
