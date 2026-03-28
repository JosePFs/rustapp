use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::create_exercise::CreateExerciseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseRequest {
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
}

pub async fn create_exercise(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateExerciseRequest>,
) -> Result<APIResponse<ExerciseResponse>> {
    let result = state
        .backoffice_facade()
        .create_exercise(CreateExerciseArgs {
            name: request.name,
            description: request.description,
            video_url: request.video_url,
            order_index: request.order_index.unwrap_or(0),
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(ExerciseResponse {
        id: result.id.to_string(),
        name: result.name.to_string(),
        description: result.description.map(|d| d.to_string()),
        video_url: result.video_url.map(|v| v.to_string()),
    }))
}
