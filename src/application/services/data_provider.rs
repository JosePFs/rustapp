use async_trait::async_trait;

use crate::domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
    WorkoutSession,
};
use crate::domain::profile::Profile;

#[async_trait(?Send)]
pub trait DataProvider: Send + Sync {
    async fn get_profiles_by_ids(
        &self,
        ids: &[String],
        access_token: &str,
    ) -> Result<Vec<Profile>, String>;

    async fn get_patient_id_by_email(
        &self,
        access_token: &str,
        email: &str,
    ) -> Result<Option<String>, String>;

    async fn list_specialist_patients(
        &self,
        access_token: &str,
    ) -> Result<Vec<SpecialistPatient>, String>;

    async fn list_programs(&self, access_token: &str) -> Result<Vec<Program>, String>;

    async fn get_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Option<Program>, String>;

    async fn list_workout_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Workout>, String>;

    async fn get_workouts_by_ids(
        &self,
        access_token: &str,
        ids: &[String],
    ) -> Result<Vec<Workout>, String>;

    async fn list_workouts_for_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<Workout>, String>;

    async fn list_program_schedule(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<ProgramScheduleItem>, String>;

    async fn list_exercises_for_workout(
        &self,
        access_token: &str,
        workout_id: &str,
    ) -> Result<Vec<Exercise>, String>;

    async fn list_exercise_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Exercise>, String>;

    async fn list_patient_programs_for_specialist(
        &self,
        access_token: &str,
    ) -> Result<Vec<PatientProgram>, String>;

    async fn get_patient_program_by_id(
        &self,
        access_token: &str,
        id: &str,
    ) -> Result<Option<PatientProgram>, String>;

    async fn list_workout_sessions(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<WorkoutSession>, String>;

    async fn list_active_patient_programs(
        &self,
        access_token: &str,
    ) -> Result<Vec<PatientProgram>, String>;
}
