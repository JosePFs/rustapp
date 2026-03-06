use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum DomainError {
    Login,
    Api(String),
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::Login => write!(f, "Invalid login credentials"),
            DomainError::Api(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl From<String> for DomainError {
    fn from(value: String) -> Self {
        DomainError::Api(value)
    }
}

impl Error for DomainError {}

pub type Result<T> = std::result::Result<T, DomainError>;
