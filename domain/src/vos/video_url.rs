use std::fmt::{Display, Formatter};

use crate::error::{DomainError, Result};

const MAX_LEN: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoUrl(String);

impl VideoUrl {
    pub fn new(value: String) -> Result<Self> {
        let t = value.trim();
        if t.is_empty() {
            return Err(DomainError::InvalidParameter(
                "video_url".to_string(),
                "(empty)".to_string(),
            ));
        }
        if t.len() > MAX_LEN {
            return Err(DomainError::InvalidParameter(
                "video_url".to_string(),
                format!("len {}", t.len()),
            ));
        }
        Ok(Self(t.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for VideoUrl {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.to_string())
    }
}

impl TryFrom<String> for VideoUrl {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl Display for VideoUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_url_new() {
        let video_url =
            VideoUrl::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()).unwrap();

        assert_eq!(
            video_url.value(),
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        );
    }

    #[test]
    fn video_url_try_from_str() {
        let video_url = VideoUrl::try_from("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();

        assert_eq!(
            video_url.value(),
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        );
    }

    #[test]
    fn video_url_try_from_string() {
        let video_url =
            VideoUrl::try_from("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()).unwrap();

        assert_eq!(
            video_url.value(),
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        );
    }

    #[test]
    fn video_url_max_length() {
        let result = VideoUrl::try_from("a".repeat(MAX_LEN + 1));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("video_url".to_string(), format!("len {}", MAX_LEN + 1))
        );
    }

    #[test]
    fn video_url_empty() {
        let result = VideoUrl::try_from("".to_string());

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::InvalidParameter("video_url".to_string(), "(empty)".to_string())
        );
    }
}
