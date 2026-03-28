use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::list_workout_library::ListWorkoutLibraryArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkoutLibraryRequest {
    pub name_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutLibraryItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

pub async fn list_workout_library(
    State(state): State<Arc<AppState>>,
    Query(request): Query<ListWorkoutLibraryRequest>,
) -> Result<APIResponse<Vec<WorkoutLibraryItemResponse>>> {
    let result = state
        .backoffice_facade()
        .list_workout_library(ListWorkoutLibraryArgs {
            name_filter: request.name_filter,
        })
        .await
        .map_err(Error::from)?;

    let workouts = result
        .into_iter()
        .map(|w| WorkoutLibraryItemResponse {
            id: w.id,
            name: w.name,
            description: w.description,
            order_index: w.order_index,
        })
        .collect();

    Ok(APIResponse::ok(workouts))
}
