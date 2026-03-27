use std::sync::Arc;

use axum::extract::State;

use crate::{router::api_response::APIResponse, state::AppState};

pub async fn refresh_session(State(state): State<Arc<AppState>>) -> APIResponse<String> {
    tracing::info!("Refresh session");
    APIResponse::ok("OK".to_string())
}
