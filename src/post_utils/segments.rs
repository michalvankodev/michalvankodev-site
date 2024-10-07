use std::{fmt::Display, rc::Rc};

use crate::blog_posts::blog_post_model::{BlogPostMetadata, Segment};

use super::post_parser::ParseResult;

impl Segment {
    fn as_str(&self) -> &'static str {
        match self {
            Segment::Blog => "blog",
            Segment::Broadcasts => "broadcasts",
            Segment::Featured => "featured",
            Segment::Cookbook => "cookbook",
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn get_posts_by_segment(
    post_list: Vec<ParseResult<BlogPostMetadata>>,
    segments: &[Segment],
) -> Vec<ParseResult<BlogPostMetadata>> {
    let mut filtered_posts: Vec<ParseResult<BlogPostMetadata>> = post_list
        .into_iter()
        .filter(|post| {
            segments
                .iter()
                .all(|segment| post.metadata.segments.contains(segment))
        }) // Filter by segments
        .filter(|post| post.metadata.published) // Filter only published posts
        .collect();

    // Sort by date in descending order
    filtered_posts.sort_by_key(|post| post.metadata.date);
    filtered_posts.reverse();

    filtered_posts
}

pub fn ref_get_posts_by_segment(
    post_list: &[Rc<ParseResult<BlogPostMetadata>>],
    segments: &[Segment],
) -> Vec<Rc<ParseResult<BlogPostMetadata>>> {
    let mut filtered_posts: Vec<Rc<ParseResult<BlogPostMetadata>>> = post_list
        .iter() // Use iter() to borrow instead of consuming the original vector
        .filter(|post| {
            let post = post.as_ref();
            segments
                .iter()
                .all(|segment| post.metadata.segments.contains(segment))
        }) // Filter by segments
        .filter(|post| post.metadata.published) // Filter only published posts
        .cloned()
        .collect(); // Collect references to ParseResult<BlogPostMetadata>

    // Sort by date in descending order
    filtered_posts.sort_by_key(|post| post.metadata.date);
    filtered_posts.reverse();

    filtered_posts
}
