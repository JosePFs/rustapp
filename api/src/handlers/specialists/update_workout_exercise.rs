use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::update_workout_exercise::UpdateWorkoutExerciseArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkoutExerciseRequest {
    pub workout_id: String,
    pub exercise_id: String,
    pub sets: i32,
    pub reps: i32,
    pub order_index: Option<i32>,
}

pub async fn update_workout_exercise(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateWorkoutExerciseRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .update_workout_exercise(UpdateWorkoutExerciseArgs {
            workout_id: request.workout_id,
            exercise_id: request.exercise_id,
            sets: request.sets,
            reps: request.reps,
            order_index: request.order_index,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
