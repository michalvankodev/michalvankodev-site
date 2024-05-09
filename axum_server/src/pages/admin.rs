use askama::Template;

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminPageTemplate {}

pub async fn render_admin() -> AdminPageTemplate {
    AdminPageTemplate {}
}
