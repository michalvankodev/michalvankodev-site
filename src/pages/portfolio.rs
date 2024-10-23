use askama::Template;
use axum::http::StatusCode;
use serde::Deserialize;

use crate::{
    components::site_header::HeaderProps,
    filters,
    post_utils::{
        post_listing::get_post_list,
        post_parser::{parse_post, ParseResult},
    },
    projects::project_model::ProjectMetadata,
};

use super::contact::ContactLink;

#[derive(Deserialize, Debug)]
pub struct Workplace {
    pub name: String,
    pub thumbnail: Option<String>,
    pub description: String,
    pub displayed: bool,
}

#[derive(Deserialize, Debug)]
pub struct Education {
    pub name: String,
    pub thumbnail: Option<String>,
    pub description: String,
    pub displayed: bool,
}

#[derive(Deserialize, Debug)]
pub struct PortfolioPageModel {
    // pub title: String,
    pub work_history: Vec<Workplace>,
    pub education: Vec<Education>,
}

#[derive(Template)]
#[template(path = "portfolio.html")]
pub struct PortfolioTemplate {
    pub title: String,
    pub body: String,
    pub project_list: Vec<ParseResult<ProjectMetadata>>,
    pub header_props: HeaderProps,
    pub workplace_list: Vec<Workplace>,
    pub education_list: Vec<Education>,
    pub contact_links: Vec<ContactLink>,
    pub technology_list: Vec<String>,
}

pub async fn render_portfolio() -> Result<PortfolioTemplate, StatusCode> {
    let portfolio = parse_post::<PortfolioPageModel>("_pages/portfolio.md").await?;

    let mut project_list = get_post_list::<ProjectMetadata>("_projects").await?;

    project_list.sort_by_key(|post| post.slug.to_string());
    project_list.retain(|project| project.metadata.displayed);
    project_list.reverse();

    let workplace_list = portfolio
        .metadata
        .work_history
        .into_iter()
        .filter(|workplace| workplace.displayed)
        .collect::<Vec<Workplace>>();

    let education_list = portfolio
        .metadata
        .education
        .into_iter()
        .filter(|education| education.displayed)
        .collect::<Vec<Education>>();

    let contact_links = vec![
        ContactLink {
            href: "mailto:michalvankosk@gmail.com".to_string(),
            label: "michalvankosk@gmail.com".to_string(),
            title: "E-mail address".to_string(),
            svg: "mail".to_string(),
        },
        ContactLink {
            href: "tel:+421-905-372-947".to_string(),
            label: "+421 905 372 947".to_string(),
            title: "Phone".to_string(),
            svg: "phone".to_string(),
        },
        ContactLink {
            href: "https://github.com/michalvankodev".to_string(),
            label: "GitHub".to_string(),
            title: "Github profile".to_string(),
            svg: "github".to_string(),
        },
        ContactLink {
            href: "https://www.linkedin.com/in/michal-vanko-dev/".to_string(),
            label: "LinkedIn".to_string(),
            title: "LinkedIn profile".to_string(),
            svg: "linkedin".to_string(),
        },
    ];

    let technology_list = vec![
        "Rust",
        "HTMX",
        "React",
        "Svelte",
        "Angular",
        "PostgreSQL",
        "Redis",
        "GraphQL",
        "TypeScript",
        "Node.js",
        "Axum",
        "Bevy",
        "Tailwind",
        "OCaml",
        "Python",
        "git",
        "Linux",
        "Docker",
        "Devops",
        "Selfhosting",
    ]
    .into_iter()
    .map(|str| str.to_owned())
    .collect();

    Ok(PortfolioTemplate {
        title: "Portfolio".to_owned(),
        body: portfolio.body,
        header_props: HeaderProps::default(),
        project_list,
        workplace_list,
        education_list,
        contact_links,
        technology_list,
    })
}
