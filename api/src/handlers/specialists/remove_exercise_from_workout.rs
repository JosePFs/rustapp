use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveExerciseFromWorkoutRequest {
    pub workout_id: String,
    pub exercise_id: String,
}

pub async fn remove_exercise_from_workout(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RemoveExerciseFromWorkoutRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .remove_exercise_from_workout(RemoveExerciseFromWorkoutArgs {
            workout_id: request.workout_id,
            exercise_id: request.exercise_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
