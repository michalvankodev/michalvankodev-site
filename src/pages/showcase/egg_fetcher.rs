use askama::Template;
use axum::http::StatusCode;

use crate::components::site_header::HeaderProps;

#[derive(Template)]
#[template(path = "egg_fetcher_page.html")]
pub struct EggFetcherShowcaseTemplate {
    header_props: HeaderProps,
}

pub async fn render_egg_fetcher() -> Result<EggFetcherShowcaseTemplate, StatusCode> {
    Ok(EggFetcherShowcaseTemplate {
        header_props: HeaderProps::default(),
    })
}
