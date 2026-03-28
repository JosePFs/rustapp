use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use infrastructure::api::{
    config::ApiConfig,
    mobile_client::{
        LoginResponse as ApiLoginResponse, MobileClient,
        PatientProgramResponse as ApiPatientProgramResponse,
    },
    platforms_http_client::PlatformsHttpClient,
};

static API_CLIENT: LazyLock<MobileClient> = LazyLock::new(|| {
    let config = ApiConfig::from_env();
    MobileClient::new(config, PlatformsHttpClient)
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub user_profile_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExerciseInstructionSummary {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgramDaySummary {
    pub day_index: i32,
    pub day_number: i32,
    pub session_id: Option<String>,
    pub workout_name: Option<String>,
    pub workout_description: Option<String>,
    pub is_rest_day: bool,
    pub session_date: Option<String>,
    pub completed_at: Option<String>,
    pub exercises: Vec<ExerciseInstructionSummary>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExerciseFeedbackInput {
    pub exercise_id: String,
    pub effort: i32,
    pub pain: i32,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkDayAsCompletedRequest {
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback: Vec<ExerciseFeedbackInput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkDayAsUncompletedRequest {
    pub workout_session_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientProgramSummary {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub days: Vec<ProgramDaySummary>,
    pub progress_percent: i32,
    pub average_effort: Option<f32>,
    pub average_pain: Option<f32>,
}

pub async fn login(request: LoginRequest) -> Result<LoginResponse, String> {
    let result: ApiLoginResponse = API_CLIENT.login(&request.email, &request.password).await?;
    Ok(LoginResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        user_profile_type: result.user_profile_type,
    })
}

pub async fn refresh_session(refresh_token: String) -> Result<LoginResponse, String> {
    let result: ApiLoginResponse = API_CLIENT.refresh_session(&refresh_token).await?;
    Ok(LoginResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        user_id: result.user_id,
        user_profile_type: result.user_profile_type,
    })
}

pub async fn get_patient_programs() -> Result<Vec<PatientProgramSummary>, String> {
    let result: Vec<ApiPatientProgramResponse> = API_CLIENT.get_programs().await?;
    Ok(result
        .into_iter()
        .map(|program| PatientProgramSummary {
            patient_program_id: program.patient_program_id,
            program_id: program.program_id,
            program_name: program.program_name,
            program_description: program.program_description,
            progress_percent: program.progress_percent,
            average_effort: program.average_effort,
            average_pain: program.average_pain,
            days: program
                .days
                .into_iter()
                .map(|day| ProgramDaySummary {
                    session_id: day.session_id,
                    day_index: day.day_index,
                    day_number: day.day_number,
                    workout_name: day.workout_name,
                    workout_description: day.workout_description,
                    is_rest_day: day.is_rest_day,
                    session_date: day.session_date,
                    completed_at: day.completed_at,
                    exercises: day
                        .exercises
                        .into_iter()
                        .map(|e| ExerciseInstructionSummary {
                            exercise_id: e.exercise_id,
                            name: e.name,
                            description: e.description,
                            video_url: e.video_url,
                            sets: e.sets,
                            reps: e.reps,
                            effort: e.effort,
                            pain: e.pain,
                            comment: e.comment,
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect())
}

pub async fn mark_day_as_completed(request: MarkDayAsCompletedRequest) -> Result<(), String> {
    let feedback: Vec<(String, i32, i32, String)> = request
        .feedback
        .into_iter()
        .map(|f| {
            (
                f.exercise_id,
                f.effort,
                f.pain,
                f.comment.unwrap_or_default(),
            )
        })
        .collect();

    API_CLIENT
        .mark_day_as_completed(
            &request.patient_program_id,
            request.day_index,
            &request.session_date,
            feedback,
        )
        .await
}

pub async fn mark_day_as_uncompleted(request: MarkDayAsUncompletedRequest) -> Result<(), String> {
    API_CLIENT
        .mark_day_as_uncompleted(&request.workout_session_id)
        .await
}

pub fn init_logger(level: String) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    };

    let _ = env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .try_init();
}
