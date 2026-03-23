use std::sync::{Arc, LazyLock};

use infrastructure::supabase::default_auth;
use serde::{Deserialize, Serialize};

use application::facade::MobileFacade;
use application::ports::api::MobileApi;
use application::use_cases::{
    get_patient_programs::GetPatientProgramsUseCase,
    login::LoginUseCaseArgs,
    mobile_login::MobileLoginUseCase,
    refresh_session::{RefreshSessionArgs, RefreshSessionUseCase},
    submit_patient_workout_feedback::{
        SubmitPatientWorkoutFeedbackArgs, SubmitPatientWorkoutFeedbackUseCase,
    },
    uncomplete_patient_workout_session::{
        UncompletePatientWorkoutSessionArgs, UncompletePatientWorkoutSessionUseCase,
    },
};
use infrastructure::supabase::{
    auth::SupabaseAuth,
    repositories::{SupabaseRestRepository, SupabaseRestRepositoryBuilder},
};

static REPOSITORY: LazyLock<Arc<SupabaseRestRepository>> =
    LazyLock::new(|| Arc::new(SupabaseRestRepositoryBuilder::new().build()));

static FACADE: LazyLock<Arc<MobileFacade<SupabaseRestRepository, SupabaseAuth>>> =
    LazyLock::new(|| {
        let repo = REPOSITORY.clone();
        let auth = default_auth();
        Arc::new(MobileFacade {
            login_uc: Arc::new(MobileLoginUseCase::new(repo.clone(), auth.clone())),
            refresh_session_uc: Arc::new(RefreshSessionUseCase::new(repo.clone(), auth.clone())),
            get_patient_programs_uc: Arc::new(GetPatientProgramsUseCase::new(repo.clone())),
            submit_patient_workout_feedback_uc: Arc::new(SubmitPatientWorkoutFeedbackUseCase::new(
                repo.clone(),
            )),
            uncomplete_patient_workout_session_uc: Arc::new(
                UncompletePatientWorkoutSessionUseCase::new(repo),
            ),
        })
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
    let result = FACADE
        .login(LoginUseCaseArgs::from(&request.email, &request.password))
        .await
        .map_err(|error| error.to_string())?;

    Ok(LoginResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: result.user_profile_type.to_string(),
    })
}

pub async fn refresh_session(refresh_token: String) -> Result<LoginResponse, String> {
    let result = FACADE
        .refresh_session(RefreshSessionArgs::from_refresh_token(refresh_token))
        .await
        .map_err(|error| error.to_string())?;

    Ok(LoginResponse {
        access_token: result.session.access_token().to_string(),
        refresh_token: result.session.refresh_token().map(|t| t.to_string()),
        user_id: result.session.user_id().to_string(),
        user_profile_type: result.user_profile_type.to_string(),
    })
}

pub async fn get_patient_programs() -> Result<Vec<PatientProgramSummary>, String> {
    let result = FACADE
        .get_patient_programs()
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

pub async fn mark_day_as_completed(request: MarkDayAsCompletedRequest) -> Result<(), String> {
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

    FACADE
        .submit_patient_workout_feedback(SubmitPatientWorkoutFeedbackArgs {
            patient_program_id,
            day_index,
            session_date,
            feedback_map,
            completion_status: None,
        })
        .await
        .map_err(|error| error.to_string())
}

pub async fn mark_day_as_uncompleted(request: MarkDayAsUncompletedRequest) -> Result<(), String> {
    FACADE
        .uncomplete_patient_workout_session(UncompletePatientWorkoutSessionArgs {
            workout_session_id: request.workout_session_id,
        })
        .await
        .map_err(|error| error.to_string())
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
