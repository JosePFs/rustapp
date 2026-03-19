use std::sync::Arc;

use crate::ports::Backend;
use domain::entities::PatientProgram;
use domain::error::Result;

#[derive(Clone)]
pub struct AssignProgramToPatientArgs {
    pub token: String,
    pub patient_id: String,
    pub program_id: String,
}

pub struct AssignProgramToPatientUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> AssignProgramToPatientUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: AssignProgramToPatientArgs) -> Result<PatientProgram> {
        self.backend
            .assign_program_to_patient(&args.token, &args.patient_id, &args.program_id)
            .await
    }
}
