use crate::error::Result;
use std::collections::HashMap;
use crate::ports::auth::Session;

pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserProfileType {
    Specialist,
    Patient,
}

impl From<&str> for UserProfileType {
    fn from(user_profile_type: &str) -> Self {
        match user_profile_type {
            "specialist" => Self::Specialist,
            "patient" => Self::Patient,
            _ => Self::Patient,
        }
    }
}

pub struct LoginResult {
    pub session: Session,
    pub user_profile_type: UserProfileType,
}

pub struct RefreshSessionArgs {
    pub refresh_token: String,
}

#[derive(Clone, PartialEq)]
pub struct ExerciseInstruction {
    pub exercise_id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: i32,
    pub reps: i32,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ProgramDay {
    pub session_id: Option<String>,
    pub day_index: i32,
    pub day_number: i32,
    pub workout_name: Option<String>,
    pub workout_description: Option<String>,
    pub is_rest_day: bool,
    pub session_date: Option<String>,
    pub completed_at: Option<String>,
    pub exercises: Vec<ExerciseInstruction>,
}

#[derive(Clone, PartialEq)]
pub struct PatientProgram {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub days: Vec<ProgramDay>,
    pub progress_percent: i32,
    pub average_effort: Option<f32>,
    pub average_pain: Option<f32>,
}

pub struct GetPatientProgramsResult {
    pub patient_programs: Vec<PatientProgram>,
}

pub struct SubmitPatientWorkoutFeedbackArgs {
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback_map: HashMap<String, (i32, i32, String)>,
    pub completion_status: Option<bool>,
}

pub struct UncompletePatientWorkoutSessionArgs {
    pub workout_session_id: String,
}

#[common::async_trait_platform]
pub trait MobileApi {
    async fn login(&self, args: LoginArgs) -> Result<LoginResult>;

    async fn refresh_session(&self, args: RefreshSessionArgs) -> Result<LoginResult>;

    async fn get_patient_programs(&self) -> Result<GetPatientProgramsResult>;

    async fn submit_patient_workout_feedback(
        &self,
        args: SubmitPatientWorkoutFeedbackArgs,
    ) -> Result<()>;

    async fn uncomplete_patient_workout_session(
        &self,
        args: UncompletePatientWorkoutSessionArgs,
    ) -> Result<()>;
}
