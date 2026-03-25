use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::entities::SpecialistPatient;
use domain::error::DomainError;
use domain::repositories::{AddSpecialistPatient, GetPatientIdByEmailRead};
use domain::vos::email::Email;

#[derive(Clone)]
pub struct AddSpecialistPatientArgs {
    pub patient_email: String,
}

pub struct AddSpecialistPatientUseCase<C>
where
    C: GetPatientIdByEmailRead + AddSpecialistPatient,
{
    catalog: Arc<C>,
}

impl<C> AddSpecialistPatientUseCase<C>
where
    C: GetPatientIdByEmailRead + AddSpecialistPatient,
{
    pub fn new(catalog: Arc<C>) -> Self {
        Self { catalog }
    }

    pub async fn execute(&self, args: AddSpecialistPatientArgs) -> Result<SpecialistPatient> {
        let email = Email::try_from(args.patient_email)?;
        let patient_id = self
            .catalog
            .get_patient_id_by_email(&email)
            .await
            .map_err(ApplicationError::from)?
            .ok_or_else(|| DomainError::Api("Patient not found".into()))?;

        self.catalog
            .add_specialist_patient(&patient_id)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use domain::error::Result;
    use domain::vos::id::Id;

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
        async fn get_patient_id_by_email(&self, _email: &Email) -> Result<Option<Id>> {
            Ok(Some(self.patient_id.clone()))
        }
    }

    #[common::async_trait_platform]
    impl AddSpecialistPatient for MockAddSpecialistPatientWrite {
        async fn add_specialist_patient(&self, _patient_id: &Id) -> Result<SpecialistPatient> {
            Ok(self.link.clone())
        }
    }
}
