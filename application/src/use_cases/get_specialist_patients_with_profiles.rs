use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::SpecialistPatient;
use crate::domain::error::Result;
use crate::domain::profile::Profile;

#[derive(Clone)]
pub struct GetSpecialistPatientsWithProfilesArgs {
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct GetSpecialistPatientsWithProfilesResult {
    pub links: Vec<SpecialistPatient>,
    pub profiles: Vec<Profile>,
}

pub struct GetSpecialistPatientsWithProfilesUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> GetSpecialistPatientsWithProfilesUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult> {
        let token = args.token;
        let links = self.backend.list_specialist_patients(&token).await?;
        let ids: Vec<String> = links.iter().map(|l| l.patient_id.clone()).collect();
        let profiles = self.backend.get_profiles_by_ids(&ids, &token).await?;
        Ok(GetSpecialistPatientsWithProfilesResult { links, profiles })
    }
}
