use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::list_program_schedule::ListProgramScheduleArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProgramScheduleRequest {
    pub program_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramScheduleEntryResponse {
    pub id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutListResponse {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramScheduleDataResponse {
    pub schedule: Vec<ProgramScheduleEntryResponse>,
    pub workouts: Vec<WorkoutListResponse>,
}

pub async fn list_program_schedule(
    State(state): State<Arc<AppState>>,
    Query(request): Query<ListProgramScheduleRequest>,
) -> Result<APIResponse<ProgramScheduleDataResponse>> {
    let result = state
        .backoffice_facade()
        .list_program_schedule(ListProgramScheduleArgs {
            program_id: request.program_id,
        })
        .await
        .map_err(Error::from)?;

    let schedule = result
        .schedule
        .into_iter()
        .map(|i| ProgramScheduleEntryResponse {
            id: i.id,
            order_index: i.order_index,
            workout_id: i.workout_id,
            days_count: i.days_count,
        })
        .collect();

    let workouts = result
        .workouts
        .into_iter()
        .map(|w| WorkoutListResponse {
            id: w.id,
            name: w.name,
        })
        .collect();

    Ok(APIResponse::ok(ProgramScheduleDataResponse {
        schedule,
        workouts,
    }))
}
