use async_trait::async_trait;

use crate::domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
    WorkoutSession,
};

#[async_trait(?Send)]
pub trait DataMutator: Send + Sync {
    async fn add_specialist_patient(
        &self,
        access_token: &str,
        specialist_id: &str,
        patient_id: &str,
    ) -> Result<SpecialistPatient, String>;

    async fn create_program(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Program, String>;

    async fn create_workout(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Workout, String>;

    async fn update_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        name: Option<&str>,
        description: Option<Option<&str>>,
        order_index: Option<i32>,
    ) -> Result<(), String>;

    async fn delete_workout(&self, access_token: &str, workout_id: &str) -> Result<(), String>;

    async fn create_program_schedule_item(
        &self,
        access_token: &str,
        program_id: &str,
        order_index: i32,
        workout_id: Option<&str>,
        days_count: i32,
    ) -> Result<ProgramScheduleItem, String>;

    async fn delete_program_schedule_item(
        &self,
        access_token: &str,
        schedule_id: &str,
    ) -> Result<(), String>;

    async fn create_exercise(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
        order_index: i32,
        video_url: Option<&str>,
    ) -> Result<Exercise, String>;

    async fn add_exercise_to_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        order_index: i32,
        sets: i32,
        reps: i32,
    ) -> Result<(), String>;

    async fn remove_exercise_from_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
    ) -> Result<(), String>;

    async fn update_workout_exercise(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        sets: i32,
        reps: i32,
        order_index: Option<i32>,
    ) -> Result<(), String>;

    async fn update_exercise(
        &self,
        access_token: &str,
        exercise_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        order_index: Option<i32>,
        video_url: Option<Option<&str>>,
    ) -> Result<(), String>;

    async fn soft_delete_exercise(
        &self,
        access_token: &str,
        exercise_id: &str,
    ) -> Result<(), String>;

    async fn restore_exercise(&self, access_token: &str, exercise_id: &str) -> Result<(), String>;

    async fn assign_program_to_patient(
        &self,
        access_token: &str,
        patient_id: &str,
        program_id: &str,
    ) -> Result<PatientProgram, String>;

    async fn unassign_program_from_patient(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<(), String>;

    async fn get_or_create_session(
        &self,
        access_token: &str,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
    ) -> Result<WorkoutSession, String>;

    async fn complete_session(
        &self,
        access_token: &str,
        session_id: &str,
    ) -> Result<(), String>;

    async fn update_session(
        &self,
        access_token: &str,
        session_id: &str,
        session_date: Option<&str>,
    ) -> Result<(), String>;

    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
        exercise_id: &str,
        effort: Option<i32>,
        pain: Option<i32>,
        comment: Option<&str>,
    ) -> Result<(), String>;

    async fn uncomplete_session(&self, access_token: &str, session_id: &str) -> Result<(), String>;
}
