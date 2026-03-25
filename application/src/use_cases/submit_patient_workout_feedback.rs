use std::{collections::HashMap, sync::Arc};

use futures::stream::{self, StreamExt};

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::PatientSessionRepository;
use domain::vos::id::Id;
use domain::vos::{DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate};

#[derive(Clone)]
pub struct SubmitPatientWorkoutFeedbackArgs {
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback_map: HashMap<String, (i32, i32, String)>,
    pub completion_status: Option<bool>,
}

pub struct SubmitPatientWorkoutFeedbackUseCase<P: PatientSessionRepository> {
    session_write: Arc<P>,
}

impl<P: PatientSessionRepository> SubmitPatientWorkoutFeedbackUseCase<P> {
    const MAX_CONCURRENT_REQUESTS: usize = 6;

    pub fn new(session_write: Arc<P>) -> Self {
        Self { session_write }
    }

    pub async fn execute(&self, args: SubmitPatientWorkoutFeedbackArgs) -> Result<()> {
        let patient_program_id = Id::try_from(args.patient_program_id)?;
        let day_index = DayIndex::try_from(args.day_index)?;
        let session_date = SessionDate::try_from(args.session_date)?;

        let session = self
            .session_write
            .get_or_create_session(&patient_program_id, day_index, &session_date)
            .await
            .map_err(ApplicationError::from)?;
        let session_id = session.id;

        self.session_write
            .complete_session(&session_id, &session_date)
            .await
            .map_err(ApplicationError::from)?;

        let session_id = session_id.clone();

        stream::iter(args.feedback_map.into_iter())
            .map(|(exercise_id_str, (effort, pain, comment))| {
                let session_write = self.session_write.clone();
                let session_id = session_id.clone();

                async move {
                    let exercise_id =
                        Id::try_from(exercise_id_str).map_err(ApplicationError::from)?;
                    let effort_vo =
                        EffortScore::try_from(effort).map_err(ApplicationError::from)?;
                    let pain_vo = PainScore::try_from(pain).map_err(ApplicationError::from)?;
                    let comment_vo = if comment.is_empty() {
                        None
                    } else {
                        Some(
                            FeedbackComment::try_from(comment.as_str())
                                .map_err(ApplicationError::from)?,
                        )
                    };
                    let comment_ref = comment_vo.as_ref();
                    session_write
                        .upsert_session_exercise_feedback(
                            &session_id,
                            &exercise_id,
                            Some(effort_vo),
                            Some(pain_vo),
                            comment_ref,
                        )
                        .await
                        .map_err(ApplicationError::from)
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_REQUESTS)
            .collect::<Vec<Result<()>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<()>>>()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::*;

    use domain::entities::WorkoutSession;
    use domain::error::Result;
    use domain::repositories::PatientSessionRepository;
    use domain::vos::id::Id;
    use domain::vos::{DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate};

    #[tokio::test]
    async fn submit_feedback_empty_map_still_completes_session() {
        let pp_id = Id::try_from("550e8400-e29b-41d4-a716-446655440090").unwrap();
        let session_id = Id::try_from("550e8400-e29b-41d4-a716-446655440091").unwrap();
        let session = WorkoutSession {
            id: session_id,
            patient_program_id: pp_id,
            day_index: DayIndex::ZERO,
            session_date: SessionDate::try_from("2025-01-15".to_string()).unwrap(),
            completed_at: None,
            created_at: None,
            updated_at: None,
        };
        let fake = MockPatientSessionWriteRepository::new(session);
        let uc = SubmitPatientWorkoutFeedbackUseCase::new(Arc::new(fake.clone()));

        uc.execute(SubmitPatientWorkoutFeedbackArgs {
            patient_program_id: "550e8400-e29b-41d4-a716-446655440090".to_string(),
            day_index: 0,
            session_date: "2025-01-15".to_string(),
            feedback_map: HashMap::new(),
            completion_status: None,
        })
        .await
        .unwrap();

        assert_eq!(*fake.get_or_create_calls.lock().unwrap(), 1);
        assert_eq!(*fake.complete_calls.lock().unwrap(), 1);
        assert_eq!(*fake.upsert_calls.lock().unwrap(), 0);
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
    impl PatientSessionRepository for MockPatientSessionWriteRepository {
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
