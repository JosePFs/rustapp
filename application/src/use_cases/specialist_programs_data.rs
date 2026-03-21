use std::sync::Arc;

use crate::ports::Backend;
use domain::entities::{PatientProgram, Program, SpecialistPatient};
use domain::error::Result;
use domain::vos::profile::Profile;

#[derive(Clone)]
pub struct SpecialistProgramsDataArgs {
    pub token: String,
    pub specialist_id: String,
}

#[derive(Clone, Debug)]
pub struct SpecialistProgramsDataResult {
    pub links: Vec<SpecialistPatient>,
    pub profiles: Vec<Profile>,
    pub programs: Vec<Program>,
    pub assignments: Vec<PatientProgram>,
}

pub struct SpecialistProgramsDataUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> SpecialistProgramsDataUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(
        &self,
        args: SpecialistProgramsDataArgs,
    ) -> Result<SpecialistProgramsDataResult> {
        let dashboard = self
            .backend
            .get_specialist_dashboard(&args.token, &args.specialist_id)
            .await?;

        Ok(SpecialistProgramsDataResult {
            links: dashboard.links,
            profiles: dashboard.profiles,
            programs: dashboard.programs,
            assignments: dashboard.assignments,
        })
    }
}
