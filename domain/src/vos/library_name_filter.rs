use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

const MAX_LEN: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryNameFilter(String);

impl LibraryNameFilter {
    pub fn new(value: String) -> Result<Self> {
        let t = value.trim().to_string();
        if t.is_empty() {
            return Err(DomainError::InvalidParameter(
                "name_filter".to_string(),
                "(empty)".to_string(),
            ));
        }
        if t.len() > MAX_LEN {
            return Err(DomainError::InvalidParameter(
                "name_filter".to_string(),
                format!("len {}", t.len()),
            ));
        }
        Ok(Self(t))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for LibraryNameFilter {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl TryFrom<String> for LibraryNameFilter {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for LibraryNameFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_trimmed() {
        assert!(LibraryNameFilter::new("  ".to_string()).is_err());
    }

    #[test]
    fn from_string() {
        let name_filter = LibraryNameFilter::try_from("hello".to_string()).unwrap();

        assert_eq!(name_filter.value(), "hello");
    }

    #[test]
    fn max_length() {
        let result = LibraryNameFilter::try_from("a".repeat(MAX_LEN + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter(
                "name_filter".to_string(),
                format!("len {}", MAX_LEN + 1)
            )
        );
    }
}
