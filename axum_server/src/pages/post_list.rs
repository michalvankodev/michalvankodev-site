use askama::Template;
use axum::http::StatusCode;
use tokio::fs::read_dir;
use tracing::info;

use crate::post_parser::{parse_post, ParseResult};

use super::post::PostMetadata;

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub posts: Vec<ParseResult<PostMetadata>>,
    // TODO tags and pagination
}

pub async fn render_post_list() -> Result<PostListTemplate, StatusCode> {
    let path = "../_posts/blog/";
    let dir = read_dir(path).await;
    let mut posts: Vec<ParseResult<PostMetadata>> = Vec::new();

    let mut files = match dir {
        Err(_reason) => {
            // TODO find the real reason
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(files) => files,
    };

    while let Some(file) = files
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

    Ok(PostListTemplate { posts })
}

// TODO Do we want pagination or not? Ask designer
// TODO How are we going to implement tags? The path extractor would have to make decision on wether we have a path or a blog post
// TODO Refactor `?` with `.map_err`
