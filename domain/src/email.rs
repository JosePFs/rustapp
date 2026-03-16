use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
