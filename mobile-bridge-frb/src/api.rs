use std::sync::Arc;

use serde::{Deserialize, Serialize};

use application::{
    ports::DataProviderSend,
    use_cases::{
        get_patient_programs::GetPatientProgramsUseCaseArgs,
        login::LoginUseCaseArgs,
        login::UserProfileType,
        mobile_get_patient_programs::MobileGetPatientProgramsUseCase,
        mobile_login::MobileLoginUseCase,
        mobile_submit_patient_workout_feedback::{
            MobileSubmitPatientWorkoutFeedbackArgs, MobileSubmitPatientWorkoutFeedbackUseCase,
        },
        uncomplete_patient_workout_session::UncompletePatientWorkoutSessionArgs,
        uncomplete_patient_workout_session::UncompletePatientWorkoutSessionUseCase,
    },
};
use domain::{credentials::Credentials, role::Role};
use infrastructure::supabase::{
    client::SupabaseClient, config::SupabaseConfig, native_api::NativeApi,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub url: String,
    pub anon_key: String,
}

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

fn backend(config: BridgeConfig) -> Arc<NativeApi> {
    let config = SupabaseConfig {
        url: config.url,
        anon_key: config.anon_key,
    };
    Arc::new(NativeApi::new(SupabaseClient::new(config)))
}

pub async fn login(request: LoginRequest, config: BridgeConfig) -> Result<LoginResponse, String> {
    let use_case = MobileLoginUseCase::<NativeApi>::new(backend(config));
    let result = use_case
        .execute(LoginUseCaseArgs {
            credentials: Credentials::from(&request.email, &request.password),
        })
        .await
        .map_err(|error| error.to_string())?;

    Ok(LoginResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: match result.user_profile_type {
            UserProfileType::Specialist => "specialist".to_string(),
            UserProfileType::Patient => "patient".to_string(),
        },
    })
}

pub async fn refresh_session(
    refresh_token: String,
    config: BridgeConfig,
) -> Result<LoginResponse, String> {
    let api = backend(config.clone());
    let client = SupabaseClient::new(SupabaseConfig {
        url: config.url,
        anon_key: config.anon_key,
    });
    let session = client
        .refresh_session(&refresh_token)
        .await
        .map_err(|e| e.to_string())?;

    let profiles = api
        .get_profiles_by_ids(&[session.user.id.to_string()], &session.access_token)
        .await
        .ok();
    let user_profile_type = profiles
        .and_then(|profiles| profiles.into_iter().next().map(|p| p.role().clone()))
        .map(|role| match role {
            Role::Specialist => UserProfileType::Specialist,
            Role::Patient => UserProfileType::Patient,
        })
        .unwrap_or(UserProfileType::Patient);

    Ok(LoginResponse {
        access_token: session.access_token,
        refresh_token: session.refresh_token,
        user_id: session.user.id,
        user_profile_type: match user_profile_type {
            UserProfileType::Specialist => "specialist".to_string(),
            UserProfileType::Patient => "patient".to_string(),
        },
    })
}

pub async fn get_patient_programs(
    token: String,
    config: BridgeConfig,
) -> Result<Vec<PatientProgramSummary>, String> {
    let use_case = MobileGetPatientProgramsUseCase::<NativeApi>::new(backend(config));
    let result = use_case
        .execute(GetPatientProgramsUseCaseArgs { token })
        .await
        .map_err(|error| error.to_string())?;

    Ok(result
        .patient_programs
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
                        .map(|exercise| ExerciseInstructionSummary {
                            exercise_id: exercise.exercise_id,
                            name: exercise.name,
                            description: exercise.description,
                            video_url: exercise.video_url,
                            sets: exercise.sets,
                            reps: exercise.reps,
                            effort: exercise.effort,
                            pain: exercise.pain,
                            comment: exercise.comment,
                        })
                        .collect(),
                })
                .collect(),
        })
        .collect())
}

pub async fn mark_day_as_completed(
    token: String,
    request: MarkDayAsCompletedRequest,
    config: BridgeConfig,
) -> Result<(), String> {
    let use_case = MobileSubmitPatientWorkoutFeedbackUseCase::<NativeApi>::new(backend(config));
    let feedback_map = request
        .feedback
        .into_iter()
        .map(|entry| {
            (
                entry.exercise_id,
                (entry.effort, entry.pain, entry.comment.unwrap_or_default()),
            )
        })
        .collect();

    let MarkDayAsCompletedRequest {
        patient_program_id,
        day_index,
        session_date,
        ..
    } = request;

    use_case
        .execute(MobileSubmitPatientWorkoutFeedbackArgs {
            token,
            patient_program_id,
            day_index,
            session_date,
            feedback_map,
            completion_status: None,
        })
        .await
        .map_err(|error| error.to_string())
}

pub async fn mark_day_as_uncompleted(
    token: String,
    request: MarkDayAsUncompletedRequest,
    config: BridgeConfig,
) -> Result<(), String> {
    let use_case = UncompletePatientWorkoutSessionUseCase::<NativeApi>::new(backend(config));
    use_case
        .execute(UncompletePatientWorkoutSessionArgs {
            token,
            workout_session_id: request.workout_session_id,
        })
        .await
        .map_err(|error| error.to_string())
}
