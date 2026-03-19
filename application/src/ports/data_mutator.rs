use async_trait::async_trait;

use domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
    WorkoutSession,
};
use domain::error::Result;

#[async_trait(?Send)]
pub trait DataMutator: Send + Sync {
    async fn add_specialist_patient(
        &self,
        access_token: &str,
        specialist_id: &str,
        patient_id: &str,
    ) -> Result<SpecialistPatient>;

    async fn create_program(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Program>;

    async fn create_workout(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Workout>;

    async fn update_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        name: Option<&str>,
        description: Option<Option<&str>>,
        order_index: Option<i32>,
    ) -> Result<()>;

    async fn delete_workout(&self, access_token: &str, workout_id: &str) -> Result<()>;

    async fn create_program_schedule_item(
        &self,
        access_token: &str,
        program_id: &str,
        order_index: i32,
        workout_id: Option<&str>,
        days_count: i32,
    ) -> Result<ProgramScheduleItem>;

    async fn delete_program_schedule_item(
        &self,
        access_token: &str,
        schedule_id: &str,
    ) -> Result<()>;

    async fn create_exercise(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
        order_index: i32,
        video_url: Option<&str>,
    ) -> Result<Exercise>;

    async fn add_exercise_to_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        order_index: i32,
        sets: i32,
        reps: i32,
    ) -> Result<()>;

    async fn remove_exercise_from_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
    ) -> Result<()>;

    async fn update_workout_exercise(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        sets: i32,
        reps: i32,
        order_index: Option<i32>,
    ) -> Result<()>;

    async fn update_exercise(
        &self,
        access_token: &str,
        exercise_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        order_index: Option<i32>,
        video_url: Option<Option<&str>>,
    ) -> Result<()>;

    async fn soft_delete_exercise(&self, access_token: &str, exercise_id: &str) -> Result<()>;

    async fn restore_exercise(&self, access_token: &str, exercise_id: &str) -> Result<()>;

    async fn assign_program_to_patient(
        &self,
        access_token: &str,
        patient_id: &str,
        program_id: &str,
    ) -> Result<PatientProgram>;

    async fn unassign_program_from_patient(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<()>;

    async fn get_or_create_session(
        &self,
        access_token: &str,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
    ) -> Result<WorkoutSession>;

    async fn complete_session(&self, access_token: &str, session_id: &str) -> Result<()>;

    async fn update_session(
        &self,
        access_token: &str,
        session_id: &str,
        session_date: Option<&str>,
    ) -> Result<()>;

    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
        exercise_id: &str,
        effort: Option<i32>,
        pain: Option<i32>,
        comment: Option<&str>,
    ) -> Result<()>;

    async fn uncomplete_session(&self, access_token: &str, session_id: &str) -> Result<()>;
}
