use std::{
    fmt::{Display, Formatter},
    sync::LazyLock,
};

use regex::Regex;

use crate::error::DomainError;

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    fn is_valid_email(email: &str) -> bool {
        EMAIL_REGEX.is_match(email)
    }
}

impl PartialEq<str> for Email {
    fn eq(&self, other: &str) -> bool {
        self.value() == other
    }
}

impl PartialEq<&str> for Email {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Email {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Email {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() || !Self::is_valid_email(value) {
            return Err(DomainError::InvalidParameter(
                "email".to_string(),
                value.to_string(),
            ));
        }

        Ok(Self::new(value.to_string()))
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_new() {
        let email = Email::new("test@example.com".to_string());

        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn test_email_eq() {
        let email = Email::new("test@example.com".to_string());
        let email2 = Email::new("test@example.com".to_string());

        assert_eq!(email, email2);
    }

    #[test]
    fn test_email_eq_str() {
        let email = Email::new("test@example.com".to_string());
        let str = "test@example.com";

        assert_eq!(email, str);
    }

    #[test]
    fn test_email_eq_string() {
        let email = Email::new("test@example.com".to_string());
        let string = "test@example.com".to_string();

        assert_eq!(email, string);
    }

    #[test]
    fn test_email_to_string() {
        let email = Email::new("test@example.com".to_string());

        assert_eq!(email.to_string(), "test@example.com");
    }

    #[test]
    fn test_email_is_valid_email() {
        assert!(Email::is_valid_email("test@example.com"));
        assert!(!Email::is_valid_email("invalid"));
    }

    #[test]
    fn test_email_try_from_str() {
        let email = Email::try_from("test@example.com").unwrap();

        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn test_email_try_from_string() {
        let email = Email::try_from("test@example.com".to_string()).unwrap();

        assert_eq!(email.value(), "test@example.com");
    }

    #[test]
    fn test_email_try_from_empty_string() {
        let result = Email::try_from("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("email".to_string(), "".to_string())
        );
    }

    #[test]
    fn test_email_try_from_invalid_str() {
        let result = Email::try_from("invalid");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("email".to_string(), "invalid".to_string())
        );
    }
}
