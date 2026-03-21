use async_trait::async_trait;

use crate::entities::WorkoutSession;
use crate::error::Result;
use crate::vos::id::Id;
use crate::vos::{
    AccessToken, DayIndex, EffortScore, FeedbackComment, PainScore, SessionDate,
};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PatientSessionWriteRepository: Send + Sync {
    async fn get_or_create_session(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession>;

    async fn complete_session(
        &self,
        access_token: &AccessToken,
        session_id: &Id,
        session_date: &SessionDate,
    ) -> Result<()>;

    async fn uncomplete_session(&self, access_token: &AccessToken, session_id: &Id)
        -> Result<()>;

    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &AccessToken,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()>;
}
