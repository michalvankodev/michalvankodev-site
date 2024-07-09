use chrono::{DateTime, Utc};
use tracing::debug;

// This filter does not have extra arguments
pub fn pretty_date(date_time: &DateTime<Utc>) -> ::askama::Result<String> {
    let formatted = format!("{}", date_time.format("%e %B %Y"));
    Ok(formatted)
}

// This filter does not have extra arguments
pub fn description_filter(body: &String) -> ::askama::Result<String> {
    let description = body
        .lines()
        .filter(|line| line.starts_with("<p"))
        .take(2)
        .collect::<Vec<&str>>()
        .join("\n");
    debug!(description);
    Ok(description)
}
