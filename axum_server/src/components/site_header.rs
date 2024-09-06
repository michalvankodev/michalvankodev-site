pub struct Link {
    pub href: String,
    pub label: String,
}

#[derive(Default)]
pub struct HeaderProps {
    pub back_link: Option<Link>,
}

impl HeaderProps {
    pub fn with_back_link(link: Link) -> Self {
        Self {
            back_link: Some(link),
        }
    }
}
