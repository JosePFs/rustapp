#[derive(Debug, Clone, PartialEq)]
pub struct Id(String);

impl Id {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
