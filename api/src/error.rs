use application::error::ApplicationError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;
use thiserror::Error;

use crate::router::api_response::APIResponse;

#[derive(Error, Clone, Debug, strum_macros::AsRefStr, PartialEq, Eq, Serialize)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Wrong request path: {0}, method: {1}")]
    WrongRequestPath(String, String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::WrongRequestPath(_, _) => StatusCode::NOT_FOUND,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Error::WrongRequestPath(path, method) => {
                format!("Wrong request path: {path}, method: {method}")
            }
            Error::BadRequest(message) => format!("Bad request: {message}"),
            _ => "Internal server error".to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let json = APIResponse {
            data: None::<String>,
            error: Some(self),
            status_code: status.as_u16(),
        };
        (status, Json(json)).into_response()
    }
}

impl From<ApplicationError> for Error {
    fn from(error: ApplicationError) -> Self {
        Error::BadRequest(error.to_string())
    }
}

pub type Result<T> = axum::response::Result<T, Error>;
