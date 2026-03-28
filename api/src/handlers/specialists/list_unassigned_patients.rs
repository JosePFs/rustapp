use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::list_unassigned_patients::UnassignedPatientsArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnassignedPatientsRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignedPatientResponse {
    pub patient_id: String,
    pub email: String,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignedPatientsResponse {
    pub patients: Vec<UnassignedPatientResponse>,
}

pub async fn list_unassigned_patients(
    State(state): State<Arc<AppState>>,
) -> Result<APIResponse<UnassignedPatientsResponse>> {
    let result = state
        .backoffice_facade()
        .list_unassigned_patients(UnassignedPatientsArgs {})
        .await
        .map_err(Error::from)?;

    let patients = result
        .patients
        .into_iter()
        .map(|p| UnassignedPatientResponse {
            patient_id: p.patient_id,
            email: p.email,
            full_name: p.full_name,
        })
        .collect();

    Ok(APIResponse::ok(UnassignedPatientsResponse { patients }))
}
