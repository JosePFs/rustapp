use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(value: String) -> Result<Self> {
        let t = value.trim();
        if t.is_empty() {
            return Err(DomainError::InvalidParameter(
                "access_token".to_string(),
                "(empty)".to_string(),
            ));
        }
        Ok(Self(t.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AccessToken {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty() {
        assert!(AccessToken::new("".to_string()).is_err());
        assert!(AccessToken::new("   ".to_string()).is_err());
    }

    #[test]
    fn accepts_non_empty() {
        let t = AccessToken::new("bearer-value".to_string()).unwrap();

        assert_eq!(t.value(), "bearer-value");
    }
}
