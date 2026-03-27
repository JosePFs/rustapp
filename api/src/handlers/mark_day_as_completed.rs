use std::sync::Arc;

use axum::extract::State;

use crate::{router::api_response::APIResponse, state::AppState};

pub async fn mark_day_as_completed(State(state): State<Arc<AppState>>) -> APIResponse<String> {
    tracing::info!("Mark day as completed");
    APIResponse::ok("OK".to_string())
}
