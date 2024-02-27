use chrono::{DateTime, Utc};

// This filter does not have extra arguments
pub fn pretty_date(date_time: &DateTime<Utc>) -> ::askama::Result<String> {
    let formatted = format!("{}", date_time.format("%e %B %Y"));
    Ok(formatted)
}
