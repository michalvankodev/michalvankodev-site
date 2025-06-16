use std::rc::Rc;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tokio::try_join;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, Segment, BLOG_POST_PATH},
    components::site_header::HeaderProps,
    filters,
    post_utils::{
        post_listing::get_post_list, post_parser::ParseResult, segments::ref_get_posts_by_segment,
        tags::get_popular_tags,
    },
    projects::{featured_projects::get_featured_projects, project_model::ProjectMetadata},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    header_props: HeaderProps,
    blog_tags: Vec<String>,
    broadcasts_tags: Vec<String>,
    featured_blog_posts: Vec<Rc<ParseResult<BlogPostMetadata>>>,
    featured_projects: Vec<ParseResult<ProjectMetadata>>,
    featured_broadcasts: Vec<Rc<ParseResult<BlogPostMetadata>>>,
}

pub async fn render_index() -> Result<impl IntoResponse, StatusCode> {
    let (blog_tags, broadcasts_tags, all_posts, featured_projects) = try_join!(
        get_popular_tags(Some(Segment::Blog)),
        get_popular_tags(Some(Segment::Broadcasts)),
        get_post_list::<BlogPostMetadata>(BLOG_POST_PATH),
        get_featured_projects()
    )?;

    // Convert the all_posts into Rc<ParseResult<BlogPostMetadata>>
    let all_posts_rc: Vec<Rc<ParseResult<BlogPostMetadata>>> =
        all_posts.into_iter().map(Rc::new).collect();

    let featured_blog_posts =
        ref_get_posts_by_segment(&all_posts_rc, &[Segment::Blog, Segment::Featured]);

    let featured_broadcasts =
        ref_get_posts_by_segment(&all_posts_rc, &[Segment::Broadcasts, Segment::Featured]);

    Ok(Html(
        IndexTemplate {
            header_props: HeaderProps::default(),
            blog_tags,
            broadcasts_tags,
            featured_blog_posts,
            featured_projects,
            featured_broadcasts,
        }
        .render()
        .unwrap(),
    ))
}
