use axum::http::{Method, Uri};

use crate::router::api_response::APIResponse;

pub async fn fallback(uri: Uri, req_method: Method) -> APIResponse {
    APIResponse::not_found(uri.to_string(), req_method.to_string())
}
