use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::error::Error;

#[derive(Serialize)]
pub struct APIResponse<T: Serialize = ()> {
    pub data: Option<T>,
    pub error: Option<Error>,
    pub status_code: u16,
}

impl<T> APIResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            status_code: StatusCode::OK.as_u16(),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            status_code: StatusCode::CREATED.as_u16(),
        }
    }

    pub fn no_content() -> Self {
        Self {
            data: None,
            error: None,
            status_code: StatusCode::NO_CONTENT.as_u16(),
        }
    }

    pub fn not_found(path: String, method: String) -> Self {
        APIResponse {
            data: None,
            error: Some(Error::WrongRequestPath(path, method)),
            status_code: StatusCode::NOT_FOUND.as_u16(),
        }
    }

    pub fn forbidden() -> Self {
        Self {
            data: None,
            error: None,
            status_code: StatusCode::FORBIDDEN.as_u16(),
        }
    }

    pub fn not_acceptable() -> Self {
        Self {
            data: None,
            error: None,
            status_code: StatusCode::NOT_ACCEPTABLE.as_u16(),
        }
    }

    pub fn bad_request(error: Error) -> Self {
        Self::error(error, StatusCode::BAD_REQUEST)
    }

    pub fn internal_server_error(error: Error) -> Self {
        Self::error(error, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn error(error: Error, status_code: StatusCode) -> Self {
        Self {
            data: None,
            error: Some(error),
            status_code: status_code.as_u16(),
        }
    }
}

impl<T> IntoResponse for APIResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}
