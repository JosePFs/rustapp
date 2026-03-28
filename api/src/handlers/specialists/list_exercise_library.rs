use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::list_exercise_library::ListExerciseLibraryArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExerciseLibraryRequest {
    pub name_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseLibraryItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
}

pub async fn list_exercise_library(
    State(state): State<Arc<AppState>>,
    Query(request): Query<ListExerciseLibraryRequest>,
) -> Result<APIResponse<Vec<ExerciseLibraryItemResponse>>> {
    let result = state
        .backoffice_facade()
        .list_exercise_library(ListExerciseLibraryArgs {
            name_filter: request.name_filter,
        })
        .await
        .map_err(Error::from)?;

    let exercises = result
        .into_iter()
        .map(|e| ExerciseLibraryItemResponse {
            id: e.id.to_string(),
            name: e.name.to_string(),
            description: e.description.map(|d| d.to_string()),
            video_url: e.video_url.map(|v| v.to_string()),
        })
        .collect();

    Ok(APIResponse::ok(exercises))
}
