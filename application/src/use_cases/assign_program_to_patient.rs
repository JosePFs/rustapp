use std::sync::Arc;

use domain::entities::PatientProgram;
use domain::error::Result;
use domain::repositories::AssignProgramToPatientWrite;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct AssignProgramToPatientArgs {
    pub token: String,
    pub patient_id: String,
    pub program_id: String,
}

pub struct AssignProgramToPatientUseCase<W: AssignProgramToPatientWrite> {
    catalog_write: Arc<W>,
}

impl<W: AssignProgramToPatientWrite> AssignProgramToPatientUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: AssignProgramToPatientArgs) -> Result<PatientProgram> {
        let access = AccessToken::try_from(args.token)?;
        let patient_id = Id::try_from(args.patient_id)?;
        let program_id = Id::try_from(args.program_id)?;
        self.catalog_write
            .assign_program_to_patient(&access, &patient_id, &program_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeAssignProgramToPatient;

    const TOKEN: &str = "t";
    const PAT: &str = "550e8400-e29b-41d4-a716-446655440070";
    const PRG: &str = "550e8400-e29b-41d4-a716-446655440071";

    #[tokio::test]
    async fn assign_program_forwards_ids() {
        let pp = PatientProgram {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440072").unwrap(),
            patient_id: Id::try_from(PAT).unwrap(),
            program_id: Id::try_from(PRG).unwrap(),
            status: "active".to_string(),
        };
        let fake = FakeAssignProgramToPatient::new_ok(pp.clone());
        let uc = AssignProgramToPatientUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(AssignProgramToPatientArgs {
                token: TOKEN.to_string(),
                patient_id: PAT.to_string(),
                program_id: PRG.to_string(),
            })
            .await
            .unwrap();

        assert_eq!(got.id, pp.id);

        let pair = fake.last_pair.lock().unwrap().clone().unwrap();
        assert_eq!(pair.0.to_string(), PAT);
        assert_eq!(pair.1.to_string(), PRG);
    }
}
