use std::sync::Arc;

use crate::ports::Backend;
use domain::entities::SpecialistPatient;
use domain::error::{DomainError, Result};

#[derive(Clone)]
pub struct AddSpecialistPatientArgs {
    pub token: String,
    pub specialist_id: String,
    pub patient_email: String,
}

pub struct AddSpecialistPatientUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> AddSpecialistPatientUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: AddSpecialistPatientArgs) -> Result<SpecialistPatient> {
        let patient_id = self
            .backend
            .get_patient_id_by_email(&args.token, &args.patient_email)
            .await?
            .ok_or_else(|| DomainError::Api("Patient not found".into()))?;

        self.backend
            .add_specialist_patient(&args.token, &args.specialist_id, &patient_id)
            .await
    }
}
