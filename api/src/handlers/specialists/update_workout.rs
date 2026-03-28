use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::update_workout::UpdateWorkoutArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkoutRequest {
    pub workout_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn update_workout(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateWorkoutRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .update_workout(UpdateWorkoutArgs {
            workout_id: request.workout_id,
            name: request.name,
            description: request.description,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
