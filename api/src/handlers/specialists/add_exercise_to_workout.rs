use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::add_exercise_to_workout::AddExerciseToWorkoutArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddExerciseToWorkoutRequest {
    pub workout_id: String,
    pub exercise_id: String,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

pub async fn add_exercise_to_workout(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddExerciseToWorkoutRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .add_exercise_to_workout(AddExerciseToWorkoutArgs {
            workout_id: request.workout_id,
            exercise_id: request.exercise_id,
            order_index: request.order_index,
            sets: request.sets,
            reps: request.reps,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
