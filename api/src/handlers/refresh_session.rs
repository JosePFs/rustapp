use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::{ports::MobileApi as _, use_cases::refresh_session::RefreshSessionArgs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshSessionResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub user_profile_type: String,
}

pub async fn refresh_session(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshSessionRequest>,
) -> Result<APIResponse<RefreshSessionResponse>> {
    let result = state
        .facade()
        .refresh_session(RefreshSessionArgs::from_refresh_token(&request.refresh_token))
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(RefreshSessionResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: result.user_profile_type.to_string(),
    }))
}