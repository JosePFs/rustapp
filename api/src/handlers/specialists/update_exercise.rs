use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::update_exercise::UpdateExerciseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExerciseRequest {
    pub exercise_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub order_index: Option<i32>,
}

pub async fn update_exercise(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateExerciseRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .update_exercise(UpdateExerciseArgs {
            exercise_id: request.exercise_id,
            name: request.name,
            description: request.description,
            video_url: request.video_url,
            order_index: request.order_index,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
