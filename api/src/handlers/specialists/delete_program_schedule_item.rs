use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::delete_program_schedule_item::DeleteProgramScheduleItemArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProgramScheduleItemRequest {
    pub schedule_item_id: String,
}

pub async fn delete_program_schedule_item(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteProgramScheduleItemRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .delete_program_schedule_item(DeleteProgramScheduleItemArgs {
            schedule_item_id: request.schedule_item_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
