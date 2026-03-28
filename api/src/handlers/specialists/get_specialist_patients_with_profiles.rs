use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientLinkResponse {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProfileResponse {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientWithProfileResponse {
    pub patient_id: String,
    pub email: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub program_name: Option<String>,
    pub progress_percent: Option<i32>,
    pub links: Vec<PatientLinkResponse>,
    pub profiles: Vec<PatientProfileResponse>,
}

pub async fn get_specialist_patients_with_profiles(
    State(state): State<Arc<AppState>>,
) -> Result<APIResponse<Vec<PatientWithProfileResponse>>> {
    let result = state
        .backoffice_facade()
        .get_specialist_patients_with_profiles(GetSpecialistPatientsWithProfilesArgs {})
        .await
        .map_err(Error::from)?;

    let links: Vec<PatientLinkResponse> = result
        .links
        .into_iter()
        .map(|l| PatientLinkResponse {
            link_id: l.link_id,
            patient_id: l.patient_id,
        })
        .collect();

    let profiles: Vec<PatientProfileResponse> = result
        .profiles
        .into_iter()
        .map(|p| PatientProfileResponse {
            patient_id: p.patient_id,
            full_name: p.full_name,
            email: p.email,
        })
        .collect();

    let patients: Vec<PatientWithProfileResponse> = links
        .iter()
        .zip(profiles.iter())
        .map(|(link, profile)| PatientWithProfileResponse {
            patient_id: link.patient_id.clone(),
            email: profile.email.clone(),
            full_name: Some(profile.full_name.clone()),
            phone: None,
            program_name: None,
            progress_percent: None,
            links: links.clone(),
            profiles: profiles.clone(),
        })
        .collect();

    Ok(APIResponse::ok(patients))
}
