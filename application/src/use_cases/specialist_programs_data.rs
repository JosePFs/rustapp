use std::sync::Arc;

use futures::try_join;

use crate::application::Backend;
use crate::domain::entities::{PatientProgram, Program, SpecialistPatient};
use crate::domain::error::Result;
use crate::domain::profile::Profile;

#[derive(Clone)]
pub struct SpecialistProgramsDataArgs {
    pub token: String,
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
        let token = args.token;
        let (links, programs, assignments) = try_join!(
            self.backend.list_specialist_patients(&token),
            self.backend.list_programs(&token),
            self.backend.list_patient_programs_for_specialist(&token),
        )?;
        let ids: Vec<String> = links.iter().map(|l| l.patient_id.clone()).collect();
        let profiles = self.backend.get_profiles_by_ids(&ids, &token).await?;
        Ok(SpecialistProgramsDataResult {
            links,
            profiles,
            programs,
            assignments,
        })
    }
}
