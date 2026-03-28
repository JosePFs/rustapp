use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::create_program::CreateProgramArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_program(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateProgramRequest>,
) -> Result<APIResponse<ProgramResponse>> {
    let result = state
        .backoffice_facade()
        .create_program(CreateProgramArgs {
            name: request.name,
            description: request.description,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(ProgramResponse {
        id: result.id.to_string(),
        name: result.name.to_string(),
        description: result.description.map(|d| d.to_string()),
    }))
}
