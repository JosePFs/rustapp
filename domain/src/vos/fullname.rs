use std::fmt::{Display, Formatter};

use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FullName(String);

impl FullName {
    fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for FullName {
    fn eq(&self, other: &str) -> bool {
        self.value() == other
    }
}

impl PartialEq<&str> for FullName {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for FullName {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for FullName {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(DomainError::InvalidParameter(
                "fullname".to_string(),
                value.to_string(),
            ));
        }

        Ok(Self::new(value.to_string()))
    }
}

impl TryFrom<String> for FullName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fullname_new() {
        let fullname = FullName::new("John Doe".to_string());

        assert_eq!(fullname.value(), "John Doe");
    }

    #[test]
    fn test_fullname_eq() {
        let fullname = FullName::new("John Doe".to_string());
        let fullname2 = FullName::new("John Doe".to_string());

        assert_eq!(fullname, fullname2);
    }

    #[test]
    fn test_fullname_eq_str() {
        let fullname = FullName::new("John Doe".to_string());
        let str = "John Doe";

        assert_eq!(fullname, str);
    }

    #[test]
    fn test_fullname_eq_string() {
        let fullname = FullName::new("John Doe".to_string());
        let string = "John Doe".to_string();

        assert_eq!(fullname, string);
    }

    #[test]
    fn test_fullname_to_string() {
        let fullname = FullName::new("John Doe".to_string());

        assert_eq!(fullname.to_string(), "John Doe");
    }

    #[test]
    fn test_fullname_try_from_str() {
        let fullname = FullName::try_from("John Doe").unwrap();

        assert_eq!(fullname.value(), "John Doe");
    }

    #[test]
    fn test_fullname_try_from_string() {
        let fullname = FullName::try_from("John Doe".to_string()).unwrap();

        assert_eq!(fullname.value(), "John Doe");
    }

    #[test]
    fn test_fullname_try_from_empty_string() {
        let result = FullName::try_from("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("fullname".to_string(), "".to_string())
        );
    }
}
