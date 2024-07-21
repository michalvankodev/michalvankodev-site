use askama::Template;

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

pub async fn render_site_footer() -> SiteFooter {
    let mut post_list = get_post_list::<PostMetadata>(BLOG_POST_PATH)
        .await
        .unwrap_or(vec![]);
    post_list.sort_by_key(|post| post.metadata.date);
    post_list.reverse();

    let latest_posts = post_list
        .into_iter()
        .take(6)
        .collect::<Vec<ParseResult<PostMetadata>>>();
    SiteFooter { latest_posts }
}
