use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::login::LoginUseCaseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginSpecialistRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginSpecialistResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub user_profile_type: String,
}

pub async fn login_specialist(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginSpecialistRequest>,
) -> Result<APIResponse<LoginSpecialistResponse>> {
    let result = state
        .backoffice_facade()
        .login(LoginUseCaseArgs {
            credentials: application::ports::auth::Credentials::from(
                &request.email,
                &request.password,
            ),
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(LoginSpecialistResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: result.user_profile_type.to_string(),
    }))
}
