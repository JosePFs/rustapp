use async_trait::async_trait;

use crate::domain::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutExercise,
    WorkoutSession,
};
use crate::domain::{error::Result, profile::Profile};

#[async_trait]
pub trait DataProviderSend: Send + Sync {
    async fn get_profiles_by_ids(&self, ids: &[String], access_token: &str)
        -> Result<Vec<Profile>>;

    async fn get_program(&self, access_token: &str, program_id: &str) -> Result<Option<Program>>;

    async fn list_workouts_for_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<Workout>>;

    async fn list_program_schedule(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<ProgramScheduleItem>>;

    async fn list_workout_sessions(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<WorkoutSession>>;

    async fn list_exercises_for_workout(
        &self,
        access_token: &str,
        workout_id: &str,
    ) -> Result<Vec<WorkoutExercise>>;

    async fn list_session_exercise_feedback_for_program(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>>;

    async fn list_active_patient_programs(&self, access_token: &str)
        -> Result<Vec<PatientProgram>>;
}
