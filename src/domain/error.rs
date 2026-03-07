use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum DomainError {
    Login(String),
    Api(String),
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::Login(msg) => write!(f, "{msg}"),
            DomainError::Api(msg) => write!(f, "{msg}"),
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
