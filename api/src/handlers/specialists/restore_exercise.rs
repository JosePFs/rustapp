use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::restore_exercise::RestoreExerciseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreExerciseRequest {
    pub exercise_id: String,
}

pub async fn restore_exercise(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RestoreExerciseRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .restore_exercise(RestoreExerciseArgs {
            exercise_id: request.exercise_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
