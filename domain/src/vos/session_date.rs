use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

use regex::Regex;

use crate::error::{DomainError, Result};

static ISO_DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionDate(String);

impl SessionDate {
    pub fn new(value: String) -> Result<Self> {
        let t = value.trim();
        if !ISO_DATE.is_match(t) {
            return Err(DomainError::InvalidParameter(
                "session_date".to_string(),
                t.to_string(),
            ));
        }
        Ok(Self(t.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for SessionDate {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl TryFrom<String> for SessionDate {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for SessionDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_date() {
        assert!(SessionDate::try_from("2024-01-15").is_ok());
    }

    #[test]
    fn from_string() {
        let session_date = SessionDate::try_from("2024-01-15".to_string()).unwrap();

        assert_eq!(session_date.value(), "2024-01-15");
    }

    #[test]
    fn invalid() {
        assert!(SessionDate::try_from("not-a-date").is_err());
    }
}
