use askama::Template;
use axum::{extract::Path, http::StatusCode};
use chrono::{DateTime, Utc};
use tokio::try_join;

use crate::{
    blog_posts::blog_post_model::BlogPostMetadata, components::site_header::Link, filters,
    post_utils::post_parser::parse_post,
};

use crate::components::{
    site_footer::{render_site_footer, SiteFooter},
    site_header::HeaderProps,
};

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate {
    pub title: String,
    pub body: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub site_footer: SiteFooter,
    pub header_props: HeaderProps,
}

pub async fn render_blog_post(Path(post_id): Path<String>) -> Result<BlogPostTemplate, StatusCode> {
    let path = format!("../_posts/blog/{}.md", post_id);
    let parse_post = parse_post::<BlogPostMetadata>(&path);
    let (site_footer, parsed) = try_join!(render_site_footer(), parse_post)?;

    Ok(BlogPostTemplate {
        title: parsed.metadata.title,
        date: parsed.metadata.date,
        tags: parsed.metadata.tags,
        body: parsed.body,
        site_footer,
        header_props: HeaderProps::with_back_link(Link {
            href: "/blog".to_string(),
            label: "All posts".to_string(),
        }),
    })
}
