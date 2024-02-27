pub struct Link {
    pub href: String,
    pub label: String,
}

pub struct HeaderProps {
    pub back_link: Option<Link>,
}

impl Default for HeaderProps {
    fn default() -> Self {
        Self { back_link: None }
    }
}

impl HeaderProps {
    pub fn with_back_link(link: Link) -> Self {
        Self {
            back_link: Some(link),
            ..Default::default()
        }
    }
}
