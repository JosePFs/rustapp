use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use domain::error::DomainError;

#[derive(Clone, Debug, PartialEq)]
pub struct StatusCode(pub i32);

impl StatusCode {
    pub fn is_auth_error(&self) -> bool {
        self.0 == 401 || self.0 == 403
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationError {
    DomainError(DomainError),
    Api(StatusCode, String),
    NoSession,
    NoRefreshToken,
    RefreshFailed,
}

impl ApplicationError {
    pub fn is_auth_error(&self) -> bool {
        match self {
            ApplicationError::DomainError(e) => e.is_auth_error(),
            ApplicationError::NoSession => true,
            ApplicationError::NoRefreshToken => true,
            ApplicationError::RefreshFailed => true,
            ApplicationError::Api(status, _) => status.is_auth_error(),
        }
    }

    pub fn api(status: impl Into<i32>, message: String) -> Self {
        Self::Api(StatusCode(status.into()), message)
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::DomainError(error) => write!(f, "{error}"),
            ApplicationError::Api(status, message) => {
                write!(f, "API error: {}: {}", status.0, message)
            }
            ApplicationError::NoSession => write!(f, "No session"),
            ApplicationError::NoRefreshToken => write!(f, "No refresh token"),
            ApplicationError::RefreshFailed => write!(f, "Refresh failed"),
        }
    }
}

impl From<ApplicationError> for domain::error::DomainError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::DomainError(error) => error,
            ApplicationError::Api(status, message) => {
                domain::error::DomainError::Api(format!("API error {}: {}", status.0, message))
            }
            ApplicationError::NoSession => domain::error::DomainError::AuthenticationFailed(
                "No authentication session".to_string(),
            ),
            ApplicationError::NoRefreshToken => {
                domain::error::DomainError::AuthenticationFailed("No refresh token".to_string())
            }
            ApplicationError::RefreshFailed => {
                domain::error::DomainError::AuthenticationFailed("Refresh failed".to_string())
            }
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
