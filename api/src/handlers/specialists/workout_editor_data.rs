use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::workout_editor_data::WorkoutEditorDataArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEditorDataRequest {
    pub workout_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEditorWorkoutResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEditorExerciseItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub order_index: i32,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEditorLineResponse {
    pub exercise: WorkoutEditorExerciseItemResponse,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutEditorDataResponse {
    pub workout: Option<WorkoutEditorWorkoutResponse>,
    pub exercises: Vec<WorkoutEditorLineResponse>,
    pub library: Vec<WorkoutEditorExerciseItemResponse>,
}

pub async fn workout_editor_data(
    State(state): State<Arc<AppState>>,
    Json(request): Json<WorkoutEditorDataRequest>,
) -> Result<APIResponse<WorkoutEditorDataResponse>> {
    let result = state
        .backoffice_facade()
        .workout_editor_data(WorkoutEditorDataArgs {
            workout_id: request.workout_id,
        })
        .await
        .map_err(Error::from)?;

    let workout = result.workout.map(|w| WorkoutEditorWorkoutResponse {
        id: w.id,
        name: w.name,
        description: w.description,
        order_index: w.order_index,
    });

    let exercises = result
        .exercises
        .into_iter()
        .map(|e| WorkoutEditorLineResponse {
            exercise: WorkoutEditorExerciseItemResponse {
                id: e.exercise.id,
                name: e.exercise.name,
                description: e.exercise.description,
                video_url: e.exercise.video_url,
                order_index: e.exercise.order_index,
                deleted_at: e.exercise.deleted_at,
            },
            order_index: e.order_index,
            sets: e.sets,
            reps: e.reps,
        })
        .collect();

    let library = result
        .library
        .into_iter()
        .map(|e| WorkoutEditorExerciseItemResponse {
            id: e.id,
            name: e.name,
            description: e.description,
            video_url: e.video_url,
            order_index: e.order_index,
            deleted_at: e.deleted_at,
        })
        .collect();

    Ok(APIResponse::ok(WorkoutEditorDataResponse {
        workout,
        exercises,
        library,
    }))
}
