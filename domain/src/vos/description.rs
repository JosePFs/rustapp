use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

const MAX_LEN: usize = 8000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description(String);

impl Description {
    pub fn new(value: String) -> Result<Self> {
        if value.len() > MAX_LEN {
            return Err(DomainError::InvalidParameter(
                "description".to_string(),
                format!("len {}", value.len()),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Description {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl TryFrom<String> for Description {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let d = Description::try_from("hello").unwrap();

        assert_eq!(d.value(), "hello");
    }

    #[test]
    fn from_string() {
        let d = Description::try_from("hello".to_string()).unwrap();

        assert_eq!(d.value(), "hello");
    }

    #[test]
    fn max_length() {
        let result = Description::try_from("a".repeat(MAX_LEN + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter(
                "description".to_string(),
                format!("len {}", MAX_LEN + 1)
            )
        );
    }
}
