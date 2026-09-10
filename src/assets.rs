//! Resolves the stylesheet href that templates render into `<head>`.
//!
//! Production exports (`TARGET=PROD`, set by `just prod`) use the
//! content-hashed file produced by `just tailwind_build`
//! (`styles/output.<hash>.css`) so browsers can cache it `immutable` — the
//! href changes whenever the content does. Dev (tailwind watch, livereload)
//! keeps the fixed `styles/output.css` name: no hashing, no stale lookup.

use std::sync::OnceLock;

static CSS_HREF: OnceLock<String> = OnceLock::new();

/// Used directly in templates: the stylesheet href for this server run.
pub fn css_href() -> &'static str {
    CSS_HREF.get_or_init(|| resolve_css_href(std::env::var("TARGET").ok().as_deref(), "styles"))
}

/// Dev (no TARGET) and any prod run without a hashed build both fall back to
/// the fixed name — same behavior as before hashing existed.
fn resolve_css_href(target: Option<&str>, styles_dir: &str) -> String {
    if target == Some("PROD") {
        if let Some(hashed) = find_hashed_css(styles_dir) {
            return hashed;
        }
        tracing::warn!("TARGET=PROD but no hashed stylesheet in {styles_dir:?} — run `just tailwind_build`; falling back to /styles/output.css");
    }
    "/styles/output.css".to_string()
}

/// Newest `output.<hash>.css` in the directory, as a root-relative href.
fn find_hashed_css(styles_dir: &str) -> Option<String> {
    let mut best: Option<(std::time::SystemTime, String)> = None;

    for entry in std::fs::read_dir(styles_dir).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().to_str()?.to_string();
        let is_hashed =
            name.starts_with("output.") && name.ends_with(".css") && name != "output.css";
        if !is_hashed {
            continue;
        }
        let modified = entry.metadata().ok()?.modified().ok()?;
        if best.as_ref().map_or(true, |(newest, _)| modified > *newest) {
            best = Some((modified, name));
        }
    }

    best.map(|(_, name)| format!("/styles/{name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_styles_dir(label: &str) -> String {
        let dir = std::env::temp_dir().join(format!(
            "css-href-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir.to_str().unwrap().to_string()
    }

    #[test]
    fn prod_uses_newest_hashed_stylesheet() {
        let dir = temp_styles_dir("hashed");
        std::fs::write(format!("{dir}/output.css"), "plain").unwrap();
        std::fs::write(format!("{dir}/output.aaaa1111.css"), "old").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(format!("{dir}/output.bbbb2222.css"), "new").unwrap();

        assert_eq!(
            resolve_css_href(Some("PROD"), &dir),
            "/styles/output.bbbb2222.css"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn dev_ignores_hashed_files_and_keeps_fixed_name() {
        let dir = temp_styles_dir("dev");
        std::fs::write(format!("{dir}/output.bbbb2222.css"), "hashed").unwrap();

        assert_eq!(resolve_css_href(None, &dir), "/styles/output.css");
        assert_eq!(resolve_css_href(Some("DEV"), &dir), "/styles/output.css");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn prod_without_hashed_build_falls_back() {
        let dir = temp_styles_dir("fallback");
        std::fs::write(format!("{dir}/output.css"), "plain").unwrap();

        assert_eq!(resolve_css_href(Some("PROD"), &dir), "/styles/output.css");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn non_css_and_plain_output_are_never_picked() {
        let dir = temp_styles_dir("ignore");
        std::fs::write(format!("{dir}/output.css"), "plain").unwrap();
        std::fs::write(format!("{dir}/output.aaaa1111.css.bak"), "not css").unwrap();
        std::fs::write(format!("{dir}/other.aaaa1111.css"), "wrong stem").unwrap();

        assert_eq!(resolve_css_href(Some("PROD"), &dir), "/styles/output.css");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
