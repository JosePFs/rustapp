use async_trait::async_trait;

use crate::domain::entities::WorkoutSession;
use crate::domain::error::Result;

#[async_trait]
pub trait DataMutatorSend: Send + Sync {
    async fn get_or_create_session(
        &self,
        access_token: &str,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
    ) -> Result<WorkoutSession>;

    async fn update_session(
        &self,
        access_token: &str,
        session_id: &str,
        session_date: Option<&str>,
    ) -> Result<()>;

    async fn complete_session(&self, access_token: &str, session_id: &str) -> Result<()>;

    async fn uncomplete_session(&self, access_token: &str, session_id: &str) -> Result<()>;

    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
        exercise_id: &str,
        effort: Option<i32>,
        pain: Option<i32>,
        comment: Option<&str>,
    ) -> Result<()>;
}
