use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::create_program_schedule_item::CreateProgramScheduleItemArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProgramScheduleItemRequest {
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramScheduleItemResponse {
    pub id: String,
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

pub async fn create_program_schedule_item(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateProgramScheduleItemRequest>,
) -> Result<APIResponse<ProgramScheduleItemResponse>> {
    let result = state
        .backoffice_facade()
        .create_program_schedule_item(CreateProgramScheduleItemArgs {
            program_id: request.program_id,
            order_index: request.order_index,
            workout_id: request.workout_id,
            days_count: request.days_count.unwrap_or(7),
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(ProgramScheduleItemResponse {
        id: result.id.to_string(),
        program_id: result.program_id.to_string(),
        order_index: result.order_index.value(),
        workout_id: result.workout_id.map(|w| w.to_string()),
        days_count: result.days_count.value(),
    }))
}
