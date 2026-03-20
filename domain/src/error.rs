use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    InvalidParameter(String, String),
    Login(String),
    Api(String),
    SessionNotFound,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::InvalidParameter(param, value) => {
                write!(f, "Invalid parameter: {param}={value}")
            }
            DomainError::Login(msg) => write!(f, "{msg}"),
            DomainError::Api(msg) => write!(f, "{msg}"),
            DomainError::SessionNotFound => write!(f, "Session not found"),
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
