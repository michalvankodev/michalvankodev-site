mod markdown;
pub use markdown::{extract_headings, parse_markdown, HeadingToc};

use chrono::{DateTime, Utc};

// This filter does not have extra arguments
#[askama::filter_fn]
pub fn pretty_date(
    date_time: &DateTime<Utc>,
    _: &dyn askama::Values,
) -> ::askama::Result<String> {
    let formatted = format!("{}", date_time.format("%e %B %Y"));
    Ok(formatted)
}

const FORBIDDEN_LINES: [&str; 5] = [" ", "#", "-", "!", "<"];

// This filter requires a `rows` argument when called in templates
#[askama::filter_fn]
pub fn truncate_md(
    body: &str,
    _: &dyn askama::Values,
    rows: usize,
) -> ::askama::Result<String> {
    let description = body
        .lines()
        .filter(|line| {
            !FORBIDDEN_LINES
                .iter()
                .any(|forbidden| line.starts_with(forbidden))
                && !line.is_empty()
        })
        .take(rows)
        .collect::<Vec<&str>>()
        .join("\n");
    Ok(description)
}
