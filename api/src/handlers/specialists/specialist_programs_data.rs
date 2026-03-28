use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistPatientLinkResponse {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProfileSummaryResponse {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistProgramsDataResponse {
    pub programs: Vec<ProgramSummary>,
    pub links: Vec<SpecialistPatientLinkResponse>,
    pub profiles: Vec<PatientProfileSummaryResponse>,
    pub links_count: i32,
    pub assignments_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub async fn specialist_programs_data(
    State(state): State<Arc<AppState>>,
) -> Result<APIResponse<SpecialistProgramsDataResponse>> {
    let result = state
        .backoffice_facade()
        .specialist_programs_data()
        .await
        .map_err(Error::from)?;

    let programs = result
        .programs
        .into_iter()
        .map(|p| ProgramSummary {
            id: p.id,
            name: p.name,
            description: p.description,
        })
        .collect();

    let links: Vec<SpecialistPatientLinkResponse> = result
        .links
        .into_iter()
        .map(|l| SpecialistPatientLinkResponse {
            link_id: l.link_id,
            patient_id: l.patient_id,
        })
        .collect();

    let profiles = result
        .profiles
        .into_iter()
        .map(|p| PatientProfileSummaryResponse {
            patient_id: p.patient_id,
            full_name: p.full_name,
            email: p.email,
        })
        .collect();

    let links_count: i32 = links.len() as i32;
    let assignments_count: i32 = result.assignments.len() as i32;

    Ok(APIResponse::ok(SpecialistProgramsDataResponse {
        programs,
        links,
        profiles,
        links_count,
        assignments_count,
    }))
}