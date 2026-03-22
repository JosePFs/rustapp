use std::sync::Arc;

use domain::entities::SpecialistPatient;
use domain::error::{DomainError, Result};
use domain::repositories::{AddSpecialistPatientWrite, GetPatientIdByEmailRead};
use domain::vos::email::Email;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct AddSpecialistPatientArgs {
    pub token: String,
    pub specialist_id: String,
    pub patient_email: String,
}

pub struct AddSpecialistPatientUseCase<C>
where
    C: GetPatientIdByEmailRead + AddSpecialistPatientWrite,
{
    catalog: Arc<C>,
}

impl<C> AddSpecialistPatientUseCase<C>
where
    C: GetPatientIdByEmailRead + AddSpecialistPatientWrite,
{
    pub fn new(catalog: Arc<C>) -> Self {
        Self { catalog }
    }

    pub async fn execute(&self, args: AddSpecialistPatientArgs) -> Result<SpecialistPatient> {
        let access = AccessToken::try_from(args.token)?;
        let email = Email::try_from(args.patient_email)?;
        let patient_id = self
            .catalog
            .get_patient_id_by_email(&access, &email)
            .await?
            .ok_or_else(|| DomainError::Api("Patient not found".into()))?;

        let specialist_id = Id::try_from(args.specialist_id)?;
        self.catalog
            .add_specialist_patient(&access, &specialist_id, &patient_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use domain::entities::SpecialistPatient;
    use domain::error::Result;
    use domain::repositories::{AddSpecialistPatientWrite, GetPatientIdByEmailRead};
    use domain::vos::email::Email;
    use domain::vos::id::Id;
    use domain::vos::AccessToken;

    #[tokio::test]
    async fn add_specialist_patient_happy_path() {
        let pid = Id::try_from("550e8400-e29b-41d4-a716-446655440230").unwrap();
        let sid = Id::try_from("550e8400-e29b-41d4-a716-446655440231").unwrap();
        let lid = Id::try_from("550e8400-e29b-41d4-a716-446655440232").unwrap();
        let link = SpecialistPatient {
            id: lid,
            specialist_id: sid.clone(),
            patient_id: pid.clone(),
            created_at: None,
        };
        let fake = MockAddSpecialistPatientWrite::new(pid.clone(), link.clone());
        let uc = AddSpecialistPatientUseCase::new(Arc::new(fake));

        let got = uc
            .execute(AddSpecialistPatientArgs {
                token: "tok".to_string(),
                specialist_id: sid.to_string(),
                patient_email: "p@example.com".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(got.patient_id, link.patient_id);
    }

    #[derive(Clone)]
    struct MockAddSpecialistPatientWrite {
        patient_id: Id,
        link: SpecialistPatient,
    }

    impl MockAddSpecialistPatientWrite {
        fn new(patient_id: Id, link: SpecialistPatient) -> Self {
            Self { patient_id, link }
        }
    }

    #[common::async_trait_platform]
    impl GetPatientIdByEmailRead for MockAddSpecialistPatientWrite {
        async fn get_patient_id_by_email(
            &self,
            _access_token: &AccessToken,
            _email: &Email,
        ) -> Result<Option<Id>> {
            Ok(Some(self.patient_id.clone()))
        }
    }

    #[common::async_trait_platform]
    impl AddSpecialistPatientWrite for MockAddSpecialistPatientWrite {
        async fn add_specialist_patient(
            &self,
            _access_token: &AccessToken,
            _specialist_id: &Id,
            _patient_id: &Id,
        ) -> Result<SpecialistPatient> {
            Ok(self.link.clone())
        }
    }
}
