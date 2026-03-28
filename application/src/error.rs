use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use domain::error::DomainError;

#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationError {
    DomainError(DomainError),
    NoSession,
    NoRefreshToken,
    RefreshFailed,
    Api(String),
    Internal(String),
}

impl ApplicationError {
    pub fn is_auth_error(&self) -> bool {
        match self {
            ApplicationError::DomainError(e) => e.is_auth_error(),
            ApplicationError::NoSession => true,
            ApplicationError::NoRefreshToken => true,
            ApplicationError::RefreshFailed => true,
            ApplicationError::Api(_) => false,
            ApplicationError::Internal(_) => false,
        }
    }

    pub fn api(message: String) -> Self {
        Self::Api(message)
    }

    pub fn internal(message: String) -> Self {
        Self::Internal(message)
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::DomainError(error) => write!(f, "{error}"),
            ApplicationError::Api(message) => {
                write!(f, "API error: {message}")
            }
            ApplicationError::NoSession => write!(f, "No session"),
            ApplicationError::NoRefreshToken => write!(f, "No refresh token"),
            ApplicationError::RefreshFailed => write!(f, "Refresh failed"),
            ApplicationError::Internal(message) => write!(f, "Internal error: {message}"),
        }
    }
}

impl From<ApplicationError> for domain::error::DomainError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::DomainError(error) => error,
            ApplicationError::Api(message) => domain::error::DomainError::Api(message),
            ApplicationError::NoSession => domain::error::DomainError::AuthenticationFailed(
                "No authentication session".to_string(),
            ),
            ApplicationError::NoRefreshToken => {
                domain::error::DomainError::AuthenticationFailed("No refresh token".to_string())
            }
            ApplicationError::RefreshFailed => {
                domain::error::DomainError::AuthenticationFailed("Refresh failed".to_string())
            }
            ApplicationError::Internal(message) => domain::error::DomainError::Api(message),
        }
    }
}

impl From<domain::error::DomainError> for ApplicationError {
    fn from(error: domain::error::DomainError) -> Self {
        ApplicationError::DomainError(error)
    }
}

impl Error for ApplicationError {}

pub type Result<T> = std::result::Result<T, ApplicationError>;
