use std::{collections::HashMap, sync::Arc};

use futures::stream::{self, StreamExt};

use domain::error::Result;
use domain::repositories::PatientSessionWriteRepository;
use domain::vos::id::Id;
use domain::vos::{AccessToken, DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate};

#[derive(Clone)]
pub struct SubmitPatientWorkoutFeedbackArgs {
    pub token: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback_map: HashMap<String, (i32, i32, String)>,
    pub completion_status: Option<bool>,
}

pub struct SubmitPatientWorkoutFeedbackUseCase<P: PatientSessionWriteRepository> {
    session_write: Arc<P>,
}

impl<P: PatientSessionWriteRepository> SubmitPatientWorkoutFeedbackUseCase<P> {
    const MAX_CONCURRENT_REQUESTS: usize = 6;

    pub fn new(session_write: Arc<P>) -> Self {
        Self { session_write }
    }

    pub async fn execute(&self, args: SubmitPatientWorkoutFeedbackArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let patient_program_id = Id::try_from(args.patient_program_id)?;
        let day_index = DayIndex::try_from(args.day_index)?;
        let session_date = SessionDate::try_from(args.session_date)?;

        let session = self
            .session_write
            .get_or_create_session(&access, &patient_program_id, day_index, &session_date)
            .await?;
        let session_id = session.id;

        self.session_write
            .complete_session(&access, &session_id, &session_date)
            .await?;

        let access_for_feedback = access.clone();
        let session_id = session_id.clone();

        stream::iter(args.feedback_map.into_iter())
            .map(|(exercise_id_str, (effort, pain, comment))| {
                let session_write = self.session_write.clone();
                let access = access_for_feedback.clone();
                let session_id = session_id.clone();

                async move {
                    let exercise_id = Id::try_from(exercise_id_str)?;
                    let effort_vo = EffortScore::try_from(effort)?;
                    let pain_vo = PainScore::try_from(pain)?;
                    let comment_vo = if comment.is_empty() {
                        None
                    } else {
                        Some(FeedbackComment::try_from(comment.as_str())?)
                    };
                    let comment_ref = comment_vo.as_ref();
                    session_write
                        .upsert_session_exercise_feedback(
                            &access,
                            &session_id,
                            &exercise_id,
                            Some(effort_vo),
                            Some(pain_vo),
                            comment_ref,
                        )
                        .await
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
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakePatientSessionWrite;
    use domain::entities::WorkoutSession;

    #[tokio::test]
    async fn submit_feedback_empty_map_still_completes_session() {
        let pp_id = Id::try_from("550e8400-e29b-41d4-a716-446655440090").unwrap();
        let session_id = Id::try_from("550e8400-e29b-41d4-a716-446655440091").unwrap();
        let session = WorkoutSession {
            id: session_id,
            patient_program_id: pp_id,
            day_index: 0,
            session_date: "2025-01-15".to_string(),
            completed_at: None,
            created_at: None,
            updated_at: None,
        };
        let fake = FakePatientSessionWrite::new(session);
        let uc = SubmitPatientWorkoutFeedbackUseCase::new(Arc::new(fake.clone()));

        uc.execute(SubmitPatientWorkoutFeedbackArgs {
            token: "tok".to_string(),
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
}
