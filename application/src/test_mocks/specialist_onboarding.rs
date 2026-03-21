use domain::entities::SpecialistPatient;
use domain::error::Result;
use domain::repositories::{AddSpecialistPatientWrite, GetPatientIdByEmailRead};
use domain::vos::email::Email;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct FakeOnboardSpecialistPatient {
    pub patient_id: Id,
    pub link: SpecialistPatient,
}

impl FakeOnboardSpecialistPatient {
    pub fn new(patient_id: Id, link: SpecialistPatient) -> Self {
        Self { patient_id, link }
    }
}

#[common::async_trait_platform]
impl GetPatientIdByEmailRead for FakeOnboardSpecialistPatient {
    async fn get_patient_id_by_email(
        &self,
        _access_token: &AccessToken,
        _email: &Email,
    ) -> Result<Option<Id>> {
        Ok(Some(self.patient_id.clone()))
    }
}

#[common::async_trait_platform]
impl AddSpecialistPatientWrite for FakeOnboardSpecialistPatient {
    async fn add_specialist_patient(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
        _patient_id: &Id,
    ) -> Result<SpecialistPatient> {
        Ok(self.link.clone())
    }
}
