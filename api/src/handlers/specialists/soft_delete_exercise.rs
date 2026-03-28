use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::soft_delete_exercise::SoftDeleteExerciseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftDeleteExerciseRequest {
    pub exercise_id: String,
}

pub async fn soft_delete_exercise(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SoftDeleteExerciseRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .soft_delete_exercise(SoftDeleteExerciseArgs {
            exercise_id: request.exercise_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
