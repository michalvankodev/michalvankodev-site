use std::rc::Rc;

use crate::blog_posts::blog_post_model::BlogPostMetadata;

use super::post_parser::ParseResult;

// // TODO convert segmetns to enum, find out how to serde to enum vlaue
// pub fn get_posts_by_segment(
//     post_list: &Vec<ParseResult<BlogPostMetadata>>,
//     segments: &Vec<String>,
// ) -> Vec<ParseResult<BlogPostMetadata>> {
//     let mut filtered_posts: Vec<ParseResult<BlogPostMetadata>> = post_list
//         .iter()
//         .filter(|post| {
//             segments
//                 .iter()
//                 .all(|segment| post.metadata.segments.contains(segment))
//         }) // Filter by segments
//         .filter(|post| post.metadata.published) // Filter only published posts
//         .cloned()
//         .collect();

//     // Sort by date in descending order
//     filtered_posts.sort_by_key(|post| post.metadata.date);
//     filtered_posts.reverse();

//     filtered_posts
// }

pub fn ref_get_posts_by_segment(
    post_list: &[Rc<ParseResult<BlogPostMetadata>>],
    segments: &[String],
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
