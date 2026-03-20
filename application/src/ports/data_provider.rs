use async_trait::async_trait;

use domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback,
    SpecialistPatient, Workout, WorkoutExercise, WorkoutSession,
};
use domain::{error::Result, profile::Profile};

#[async_trait(?Send)]
pub trait DataProvider: Send + Sync {
    async fn get_profiles_by_ids(&self, ids: &[String], access_token: &str)
        -> Result<Vec<Profile>>;

    async fn get_patient_id_by_email(
        &self,
        access_token: &str,
        email: &str,
    ) -> Result<Option<String>>;

    async fn list_specialist_patients(&self, access_token: &str) -> Result<Vec<SpecialistPatient>>;

    async fn list_programs(&self, access_token: &str) -> Result<Vec<Program>>;

    async fn get_program(&self, access_token: &str, program_id: &str) -> Result<Option<Program>>;

    async fn list_workout_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Workout>>;

    async fn get_workouts_by_ids(&self, access_token: &str, ids: &[String])
        -> Result<Vec<Workout>>;

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

    async fn list_exercises_for_workout(
        &self,
        access_token: &str,
        workout_id: &str,
    ) -> Result<Vec<WorkoutExercise>>;

    async fn list_exercise_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Exercise>>;

    async fn list_patient_programs_for_specialist(
        &self,
        access_token: &str,
    ) -> Result<Vec<PatientProgram>>;

    async fn get_patient_program_by_id(
        &self,
        access_token: &str,
        id: &str,
    ) -> Result<Option<PatientProgram>>;

    async fn list_workout_sessions(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<WorkoutSession>>;

    async fn list_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>>;

    async fn list_session_exercise_feedback_for_program(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>>;

    async fn list_active_patient_programs(&self, access_token: &str)
        -> Result<Vec<PatientProgram>>;
}
