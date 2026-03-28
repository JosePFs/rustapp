use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::patient_progress::PatientProgressArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgressRequest {
    pub patient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgressResponse {
    pub full_name: String,
    pub email: String,
    pub programs: Vec<PatientProgressProgramResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgressProgramResponse {
    pub program_name: String,
    pub program_description: Option<String>,
    pub assignment_status: String,
    pub sessions_count: i32,
}

pub async fn patient_progress(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PatientProgressRequest>,
) -> Result<APIResponse<PatientProgressResponse>> {
    let result = state
        .backoffice_facade()
        .patient_progress(PatientProgressArgs {
            patient_id: request.patient_id,
        })
        .await
        .map_err(Error::from)?;

    let programs = result
        .programs_with_sessions
        .into_iter()
        .map(|p| PatientProgressProgramResponse {
            program_name: p.program_name,
            program_description: p.program_description,
            assignment_status: p.assignment_status,
            sessions_count: p.sessions.len() as i32,
        })
        .collect();

    Ok(APIResponse::ok(PatientProgressResponse {
        full_name: result.profile.full_name,
        email: result.profile.email,
        programs,
    }))
}
