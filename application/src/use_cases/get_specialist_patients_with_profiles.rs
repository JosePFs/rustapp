use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::{GetProfilesByIdsRead, ListSpecialistPatientsRead};
use domain::vos::id::Id;

#[derive(Clone)]
pub struct GetSpecialistPatientsWithProfilesArgs {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecialistPatientLink {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProfileSummary {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetSpecialistPatientsWithProfilesResult {
    pub links: Vec<SpecialistPatientLink>,
    pub profiles: Vec<PatientProfileSummary>,
}

pub struct GetSpecialistPatientsWithProfilesUseCase<
    R: GetProfilesByIdsRead + ListSpecialistPatientsRead,
> {
    catalog_read: Arc<R>,
}

impl<R: GetProfilesByIdsRead + ListSpecialistPatientsRead>
    GetSpecialistPatientsWithProfilesUseCase<R>
{
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult> {
        let _ = args;
        let links_domain = self.catalog_read.list_specialist_patients().await.map_err(ApplicationError::from)?;
        let ids: Vec<Id> = links_domain.iter().map(|l| l.patient_id.clone()).collect();
        let profiles_domain = self.catalog_read.get_profiles_by_ids(&ids).await.map_err(ApplicationError::from)?;

        let links: Vec<SpecialistPatientLink> = links_domain
            .into_iter()
            .map(|l| SpecialistPatientLink {
                link_id: l.id.to_string(),
                patient_id: l.patient_id.to_string(),
            })
            .collect();

        let profiles: Vec<PatientProfileSummary> = profiles_domain
            .into_iter()
            .map(|p| PatientProfileSummary {
                patient_id: p.id().to_string(),
                full_name: p.full_name().value().to_string(),
                email: p.email().to_string(),
            })
            .collect();

        Ok(GetSpecialistPatientsWithProfilesResult { links, profiles })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use domain::entities::SpecialistPatient;
    use domain::error::Result;
    use domain::repositories::{GetProfilesByIdsRead, ListSpecialistPatientsRead};
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;

    #[tokio::test]
    async fn maps_links_and_profiles() {
        let pid = Id::try_from("550e8400-e29b-41d4-a716-446655440060").unwrap();
        let sid = Id::try_from("550e8400-e29b-41d4-a716-446655440061").unwrap();
        let lid = Id::try_from("550e8400-e29b-41d4-a716-446655440062").unwrap();
        let patients = vec![SpecialistPatient {
            id: lid.clone(),
            specialist_id: sid.clone(),
            patient_id: pid.clone(),
            created_at: None,
        }];
        let email = Email::try_from("p@example.com").unwrap();
        let full_name = FullName::try_from("Pat").unwrap();
        let role = Role::try_from("patient").unwrap();
        let profiles = vec![Profile::new(pid.clone(), email, full_name, role)];
        let fake = MockListSpecialistPatientsRead::new_ok(patients, profiles);
        let uc = GetSpecialistPatientsWithProfilesUseCase::new(Arc::new(fake));

        let res = uc
            .execute(GetSpecialistPatientsWithProfilesArgs {})
            .await
            .unwrap();

        assert_eq!(res.links.len(), 1);
        assert_eq!(res.profiles.len(), 1);
        assert_eq!(res.links[0].patient_id, pid.to_string());
        assert_eq!(res.profiles[0].email, "p@example.com");
    }

    #[derive(Clone)]
    struct MockListSpecialistPatientsRead {
        patients: Arc<Mutex<Result<Vec<SpecialistPatient>>>>,
        profiles: Arc<Mutex<Result<Vec<Profile>>>>,
    }

    impl MockListSpecialistPatientsRead {
        fn new_ok(patients: Vec<SpecialistPatient>, profiles: Vec<Profile>) -> Self {
            Self {
                patients: Arc::new(Mutex::new(Ok(patients))),
                profiles: Arc::new(Mutex::new(Ok(profiles))),
            }
        }
    }

    #[common::async_trait_platform]
    impl ListSpecialistPatientsRead for MockListSpecialistPatientsRead {
        async fn list_specialist_patients(&self) -> Result<Vec<SpecialistPatient>> {
            self.patients.lock().unwrap().clone()
        }
    }

    #[common::async_trait_platform]
    impl GetProfilesByIdsRead for MockListSpecialistPatientsRead {
        async fn get_profiles_by_ids(&self, _ids: &[Id]) -> Result<Vec<Profile>> {
            self.profiles.lock().unwrap().clone()
        }
    }
}
