use axum::http::StatusCode;
use serde::de::DeserializeOwned;
use tokio::fs::read_dir;
use tracing::info;

use super::post_parser::{parse_post, ParseResult};

pub async fn get_post_list<'de, Metadata: DeserializeOwned>(
    path: &str,
) -> Result<Vec<ParseResult<Metadata>>, StatusCode> {
    // let path = "../_posts/blog/";
    let mut dir = read_dir(path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut posts: Vec<ParseResult<Metadata>> = Vec::new();

    while let Some(file) = dir
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let file_path = file.path();
        let file_path_str = file_path.to_str().unwrap();
        info!(":{}", file_path_str);
        let post = parse_post::<Metadata>(file_path_str, false).await?;
        posts.push(post);
    }

    if std::env::var("TARGET")
        .unwrap_or_else(|_| "DEV".to_owned())
        .eq("PROD")
    {
        posts.retain(|post| !post.slug.starts_with("dev"))
    }

    Ok(posts)
}
