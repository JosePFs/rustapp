use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::PatientSessionWriteRepository;
use domain::vos::id::Id;

#[derive(Clone)]
pub struct UncompletePatientWorkoutSessionArgs {
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
        let workout_session_id = Id::try_from(args.workout_session_id)?;
        self.session_write
            .uncomplete_session(&workout_session_id)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::entities::WorkoutSession;
    use domain::error::Result;
    use domain::repositories::PatientSessionWriteRepository;
    use domain::vos::id::Id;
    use domain::vos::{DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate};

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
        let fake = MockPatientSessionWriteRepository::new(session);
        let uc = UncompletePatientWorkoutSessionUseCase::new(Arc::new(fake.clone()));

        uc.execute(UncompletePatientWorkoutSessionArgs {
            workout_session_id: "550e8400-e29b-41d4-a716-446655440100".to_string(),
        })
        .await
        .unwrap();

        assert_eq!(*fake.uncomplete_calls.lock().unwrap(), 1);
    }

    #[derive(Clone)]
    struct MockPatientSessionWriteRepository {
        session_to_return: Arc<Mutex<Result<WorkoutSession>>>,
        get_or_create_calls: Arc<Mutex<usize>>,
        complete_calls: Arc<Mutex<usize>>,
        upsert_calls: Arc<Mutex<usize>>,
        uncomplete_calls: Arc<Mutex<usize>>,
        complete_outcome: Arc<Mutex<Result<()>>>,
        upsert_outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockPatientSessionWriteRepository {
        fn new(session: WorkoutSession) -> Self {
            Self {
                session_to_return: Arc::new(Mutex::new(Ok(session))),
                get_or_create_calls: Arc::new(Mutex::new(0)),
                complete_calls: Arc::new(Mutex::new(0)),
                upsert_calls: Arc::new(Mutex::new(0)),
                uncomplete_calls: Arc::new(Mutex::new(0)),
                complete_outcome: Arc::new(Mutex::new(Ok(()))),
                upsert_outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl PatientSessionWriteRepository for MockPatientSessionWriteRepository {
        async fn get_or_create_session(
            &self,
            _patient_program_id: &Id,
            _day_index: DayIndex,
            _session_date: &SessionDate,
        ) -> Result<WorkoutSession> {
            *self.get_or_create_calls.lock().unwrap() += 1;
            self.session_to_return.lock().unwrap().clone()
        }

        async fn complete_session(
            &self,
            _session_id: &Id,
            _session_date: &SessionDate,
        ) -> Result<()> {
            *self.complete_calls.lock().unwrap() += 1;
            self.complete_outcome.lock().unwrap().clone()
        }

        async fn uncomplete_session(&self, _session_id: &Id) -> Result<()> {
            *self.uncomplete_calls.lock().unwrap() += 1;
            Ok(())
        }

        async fn upsert_session_exercise_feedback(
            &self,
            _workout_session_id: &Id,
            _exercise_id: &Id,
            _effort: Option<EffortScore>,
            _pain: Option<PainScore>,
            _comment: Option<&FeedbackComment>,
        ) -> Result<()> {
            *self.upsert_calls.lock().unwrap() += 1;
            self.upsert_outcome.lock().unwrap().clone()
        }
    }
}
