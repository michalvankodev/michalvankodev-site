use askama::Template;
use axum::http::StatusCode;

use crate::{
    pages::post::{PostMetadata, BLOG_POST_PATH},
    post_list::get_post_list,
    post_parser::ParseResult,
};

#[derive(Template)]
#[template(path = "site_footer.html")]
pub struct SiteFooter {
    pub latest_posts: Vec<ParseResult<PostMetadata>>,
}

// TODO remove whole site footer anyway
pub async fn render_site_footer() -> Result<SiteFooter, StatusCode> {
    let mut post_list = get_post_list::<PostMetadata>(BLOG_POST_PATH).await?;
    post_list.sort_by_key(|post| post.metadata.date);
    post_list.reverse();

    let latest_posts = post_list
        .into_iter()
        .take(6)
        .collect::<Vec<ParseResult<PostMetadata>>>();
    Ok(SiteFooter { latest_posts })
}
