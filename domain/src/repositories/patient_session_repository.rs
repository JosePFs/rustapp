use crate::entities::WorkoutSession;
use crate::error::Result;
use crate::vos::id::Id;
use crate::vos::{DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate};

#[common::async_trait_platform]
pub trait PatientSessionRepository: Send + Sync {
    async fn get_or_create_session(
        &self,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession>;

    async fn complete_session(&self, session_id: &Id, session_date: &SessionDate) -> Result<()>;

    async fn uncomplete_session(&self, session_id: &Id) -> Result<()>;

    async fn upsert_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()>;
}
