use std::sync::Arc;

use axum::extract::State;

use crate::{router::api_response::APIResponse, state::AppState};

pub async fn get_programs(State(state): State<Arc<AppState>>) -> APIResponse<String> {
    tracing::info!("Get programs");
    APIResponse::ok("OK".to_string())
}
