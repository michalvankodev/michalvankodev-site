use askama::Template;
use axum::http::StatusCode;

use crate::components::site_header::HeaderProps;

pub struct ContactLink {
    pub href: String,
    pub title: String,
    pub label: String,
    pub svg: String,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactPageTemplate {
    pub title: String,
    pub header_props: HeaderProps,
    pub links: Vec<ContactLink>,
}

pub async fn render_contact() -> Result<ContactPageTemplate, StatusCode> {
    let links = vec![
        ContactLink {
            href: "mailto:michalvankosk@gmail.com".to_string(),
            label: "michalvankosk@gmail.com".to_string(),
            title: "E-mail address".to_string(),
            svg: "mail".to_string(),
        },
        ContactLink {
            href: "https://twitch.tv/michalvankodev".to_string(),
            label: "Twitch".to_string(),
            title: "Twitch channel".to_string(),
            svg: "twitch".to_string(),
        },
        ContactLink {
            href: "https://tiktok.com/@michalvankodev".to_string(),
            label: "TikTok".to_string(),
            title: "TikTok channel".to_string(),
            svg: "tiktok".to_string(),
        },
        ContactLink {
            href: "https://www.youtube.com/@michalvankodev".to_string(),
            label: "YouTube".to_string(),
            title: "YouTube channel".to_string(),
            svg: "youtube".to_string(),
        },
        ContactLink {
            href: "https://instagram.com/michalvankodev".to_string(),
            label: "Instagram".to_string(),
            title: "Instagram profile".to_string(),
            svg: "instagram".to_string(),
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
        ContactLink {
            href: "https://discord.gg/2cGg7kwZEh".to_string(),
            label: "Discord".to_string(),
            title: "Discord channel".to_string(),
            svg: "discord".to_string(),
        },
    ];

    Ok(ContactPageTemplate {
        title: "Contact".to_owned(),
        header_props: HeaderProps::default(),
        links,
    })
}
