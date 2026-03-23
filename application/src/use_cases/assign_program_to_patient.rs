use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::entities::PatientProgram;
use domain::repositories::AssignProgramToPatientWrite;
use domain::vos::id::Id;

#[derive(Clone)]
pub struct AssignProgramToPatientArgs {
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
        let patient_id = Id::try_from(args.patient_id)?;
        let program_id = Id::try_from(args.program_id)?;
        self.catalog_write
            .assign_program_to_patient(&patient_id, &program_id)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use domain::error::Result;

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
        let fake = MockAssignProgramToPatientWrite::new_ok(pp.clone());
        let uc = AssignProgramToPatientUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(AssignProgramToPatientArgs {
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

    #[derive(Clone)]
    struct MockAssignProgramToPatientWrite {
        last_pair: Arc<Mutex<Option<(Id, Id)>>>,
        outcome: Arc<Mutex<Result<PatientProgram>>>,
    }

    impl MockAssignProgramToPatientWrite {
        fn new_ok(pp: PatientProgram) -> Self {
            Self {
                last_pair: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(pp))),
            }
        }
    }

    #[common::async_trait_platform]
    impl AssignProgramToPatientWrite for MockAssignProgramToPatientWrite {
        async fn assign_program_to_patient(
            &self,
            patient_id: &Id,
            program_id: &Id,
        ) -> Result<PatientProgram> {
            *self.last_pair.lock().unwrap() = Some((patient_id.clone(), program_id.clone()));
            self.outcome.lock().unwrap().clone()
        }
    }
}
