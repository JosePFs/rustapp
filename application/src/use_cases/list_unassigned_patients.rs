use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::repositories::ListUnassignedPatientsRead;

#[derive(Clone)]
pub struct UnassignedPatientsArgs {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnassignedPatient {
    pub patient_id: String,
    pub email: String,
    pub full_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnassignedPatientsResult {
    pub patients: Vec<UnassignedPatient>,
}

pub struct ListUnassignedPatientsUseCase<R: ListUnassignedPatientsRead> {
    catalog_read: Arc<R>,
}

impl<R: ListUnassignedPatientsRead> ListUnassignedPatientsUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: UnassignedPatientsArgs) -> Result<UnassignedPatientsResult> {
        let _ = args;
        let profiles = self
            .catalog_read
            .list_unassigned_patients()
            .await
            .map_err(ApplicationError::from)?;

        let patients: Vec<UnassignedPatient> = profiles
            .into_iter()
            .map(|p| UnassignedPatient {
                patient_id: p.id().to_string(),
                email: p.email().to_string(),
                full_name: p.full_name().value().to_string(),
            })
            .collect();

        Ok(UnassignedPatientsResult { patients })
    }
}
