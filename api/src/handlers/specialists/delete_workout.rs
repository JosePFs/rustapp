use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::delete_workout::DeleteWorkoutArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWorkoutRequest {
    pub workout_id: String,
}

pub async fn delete_workout(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteWorkoutRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .delete_workout(DeleteWorkoutArgs {
            workout_id: request.workout_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
