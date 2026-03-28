use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::add_specialist_patient::AddSpecialistPatientArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSpecialistPatientRequest {
    pub patient_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistPatientResponse {
    pub id: String,
    pub specialist_id: String,
    pub patient_id: String,
    pub created_at: Option<String>,
}

pub async fn add_specialist_patient(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AddSpecialistPatientRequest>,
) -> Result<APIResponse<SpecialistPatientResponse>> {
    let result = state
        .backoffice_facade()
        .add_specialist_patient(AddSpecialistPatientArgs {
            patient_email: request.patient_email,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(SpecialistPatientResponse {
        id: result.id.to_string(),
        specialist_id: result.specialist_id.to_string(),
        patient_id: result.patient_id.to_string(),
        created_at: result.created_at,
    }))
}
