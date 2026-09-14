mod markdown;
pub use markdown::{extract_headings, parse_markdown, HeadingToc};
#[cfg(test)]
pub(crate) use markdown::paragraph_nests_block;

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

/// Machine-readable ISO 8601 for `<time datetime>` attributes.
/// chrono's `Display` (`2020-05-11 05:38:18.797 UTC`) does not satisfy
/// the HTML time-element format (space separator, " UTC" suffix).
pub fn machine_date_impl(date_time: &DateTime<Utc>) -> String {
    date_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

#[askama::filter_fn]
pub fn machine_date(
    date_time: &DateTime<Utc>,
    _: &dyn askama::Values,
) -> ::askama::Result<String> {
    Ok(machine_date_impl(date_time))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn machine_date_renders_iso8601_with_z() {
        let dt = Utc.with_ymd_and_hms(2020, 5, 11, 5, 38, 18).unwrap();
        assert_eq!(machine_date_impl(&dt), "2020-05-11T05:38:18Z");
    }

    #[test]
    fn machine_date_has_no_space_or_utc_suffix() {
        let dt = Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 59).unwrap();
        let rendered = machine_date_impl(&dt);
        assert!(!rendered.contains(' '));
        assert!(!rendered.contains("UTC"));
    }
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
