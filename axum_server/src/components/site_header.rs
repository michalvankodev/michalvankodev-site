pub struct Link {
    pub href: String,
    pub label: String,
}

pub struct HeaderProps {
    pub links: Vec<Link>,
}

impl Default for HeaderProps {
    fn default() -> Self {
        Self {
            links: vec![
                Link {
                    href: "/".to_string(),
                    label: "Introduction".to_string(),
                },
                Link {
                    href: "/blog".to_string(),
                    label: "Blog".to_string(),
                },
                Link {
                    href: "/portfolio".to_string(),
                    label: "Portfolio".to_string(),
                },
            ],
        }
    }
}
