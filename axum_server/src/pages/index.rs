use askama::Template;

use crate::components::site_footer::{render_site_footer, SiteFooter};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    site_footer: SiteFooter,
}

pub async fn render_index() -> IndexTemplate {
    let site_footer = render_site_footer().await;
    IndexTemplate { site_footer }
}
