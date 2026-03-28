use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::assign_program_to_patient::AssignProgramToPatientArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignProgramToPatientRequest {
    pub patient_id: String,
    pub program_id: String,
}

pub async fn assign_program_to_patient(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AssignProgramToPatientRequest>,
) -> Result<APIResponse<()>> {
    state
        .backoffice_facade()
        .assign_program_to_patient(AssignProgramToPatientArgs {
            patient_id: request.patient_id,
            program_id: request.program_id,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
