use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

const MAX_LEN: usize = 4000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeedbackComment(String);

impl FeedbackComment {
    pub fn new(value: String) -> Result<Self> {
        if value.len() > MAX_LEN {
            return Err(DomainError::InvalidParameter(
                "comment".to_string(),
                format!("len {}", value.len()),
            ));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for FeedbackComment {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl TryFrom<String> for FeedbackComment {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for FeedbackComment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let comment = FeedbackComment::new("a".to_string()).unwrap();

        assert_eq!(comment.value(), "a");
    }

    #[test]
    fn try_from_str() {
        let comment = FeedbackComment::try_from("a").unwrap();

        assert_eq!(comment.value(), "a");
    }

    #[test]
    fn try_from_string() {
        let comment = FeedbackComment::try_from("a".to_string()).unwrap();

        assert_eq!(comment.value(), "a");
    }

    #[test]
    fn max_length() {
        let result = FeedbackComment::try_from("a".repeat(MAX_LEN + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("comment".to_string(), format!("len {}", MAX_LEN + 1))
        );
    }
}
