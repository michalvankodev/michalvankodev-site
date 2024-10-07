// This filter does not have extra arguments

const FORBIDDEN_LINES: [&str; 5] = [" ", "#", "-", "!", "<"];

pub fn truncate_md(body: &str, rows: usize) -> ::askama::Result<String> {
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
