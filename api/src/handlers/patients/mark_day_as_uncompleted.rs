use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::uncomplete_patient_workout_session::UncompletePatientWorkoutSessionArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkDayAsUncompletedRequest {
    pub workout_session_id: String,
}

pub async fn mark_day_as_uncompleted(
    State(state): State<Arc<AppState>>,
    Json(request): Json<MarkDayAsUncompletedRequest>,
) -> Result<APIResponse<()>> {
    state
        .facade()
        .uncomplete_patient_workout_session(UncompletePatientWorkoutSessionArgs {
            workout_session_id: request.workout_session_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
