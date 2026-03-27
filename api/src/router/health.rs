use axum::{routing::get, Router};

use crate::router::api_response::APIResponse;

pub fn health_routes() -> Router<()> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> APIResponse<String> {
    APIResponse::ok("OK".to_string())
}
