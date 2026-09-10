use askama::Template;
use chrono::Datelike;

use crate::{
    blog_posts::blog_post_model::{BlogPostMetadata, Segment},
    components::site_header::HeaderProps,
    filters,
    post_utils::post_parser::ParseResult,
    projects::project_model::ProjectMetadata,
};

/// Posts grouped by publication year, newest year first.
pub struct PostYearGroup {
    pub year: i32,
    pub posts: Vec<ParseResult<BlogPostMetadata>>,
}

pub fn group_posts_by_year(posts: Vec<ParseResult<BlogPostMetadata>>) -> Vec<PostYearGroup> {
    let mut sorted = posts;
    sorted.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    let mut groups: Vec<PostYearGroup> = Vec::new();
    for post in sorted {
        let year = post.metadata.date.year();
        match groups.last_mut() {
            Some(group) if group.year == year => group.posts.push(post),
            _ => groups.push(PostYearGroup {
                year,
                posts: vec![post],
            }),
        }
    }
    groups
}

#[derive(Template)]
#[template(path = "post_list.html")]
pub struct PostListTemplate {
    pub title: String,
    pub og_title: String,
    pub segment: Segment,
    pub grouped_posts: Vec<PostYearGroup>,
    pub header_props: HeaderProps,
    pub tags: Vec<String>,
    pub featured_projects: Vec<ParseResult<ProjectMetadata>>,
    pub current_url: String,
}
