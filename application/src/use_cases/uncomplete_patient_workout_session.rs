use std::sync::Arc;

use domain::error::Result;
use domain::repositories::PatientSessionWriteRepository;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct UncompletePatientWorkoutSessionArgs {
    pub token: String,
    pub workout_session_id: String,
}

pub struct UncompletePatientWorkoutSessionUseCase<P: PatientSessionWriteRepository> {
    session_write: Arc<P>,
}

impl<P: PatientSessionWriteRepository> UncompletePatientWorkoutSessionUseCase<P> {
    pub fn new(session_write: Arc<P>) -> Self {
        Self { session_write }
    }

    pub async fn execute(&self, args: UncompletePatientWorkoutSessionArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let workout_session_id = Id::try_from(args.workout_session_id)?;
        self.session_write
            .uncomplete_session(&access, &workout_session_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakePatientSessionWrite;
    use domain::entities::WorkoutSession;

    #[tokio::test]
    async fn uncomplete_calls_repo() {
        let session = WorkoutSession {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440100").unwrap(),
            patient_program_id: Id::try_from("550e8400-e29b-41d4-a716-446655440101").unwrap(),
            day_index: 0,
            session_date: "2025-01-01".to_string(),
            completed_at: None,
            created_at: None,
            updated_at: None,
        };
        let fake = FakePatientSessionWrite::new(session);
        let uc = UncompletePatientWorkoutSessionUseCase::new(Arc::new(fake.clone()));

        uc.execute(UncompletePatientWorkoutSessionArgs {
            token: "t".to_string(),
            workout_session_id: "550e8400-e29b-41d4-a716-446655440100".to_string(),
        })
        .await
        .unwrap();

        assert_eq!(*fake.uncomplete_calls.lock().unwrap(), 1);
    }
}
