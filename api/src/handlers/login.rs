use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::{ports::MobileApi as _, use_cases::login::LoginUseCaseArgs};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub user_profile_type: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<APIResponse<LoginResponse>> {
    let result = state
        .facade()
        .login(LoginUseCaseArgs::from(&request.email, &request.password))
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(LoginResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: result.user_profile_type.to_string(),
    }))
}
