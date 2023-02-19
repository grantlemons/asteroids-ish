#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Name(pub String);

impl Name {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
