use std::sync::Arc;

use crate::application::ports::{LocalNotificationService, StubLocalNotificationService};
use crate::application::use_cases::{
    get_patient_programs::GetPatientProgramsUseCase, login::LoginUseCase,
    patient_workout_session::PatientWorkoutSessionUseCase,
    submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackUseCase,
    uncomplete_patient_workout_session::UncompletePatientWorkoutSessionUseCase,
};
use crate::domain::error::Result;
use crate::infrastructure::{
    app_context::AppContext,
    supabase::{api::Api, client::SupabaseClient, config::SupabaseConfig},
};

fn local_notifications_impl() -> Arc<dyn LocalNotificationService> {
    #[cfg(target_os = "android")]
    {
        Arc::new(crate::infrastructure::android::notifications::AndroidLocalNotifications::new())
    }
    #[cfg(not(target_os = "android"))]
    {
        Arc::new(StubLocalNotificationService::default())
    }
}

pub fn build_app_context() -> Result<AppContext> {
    let config = SupabaseConfig::from_env()?;

    let api = Api::new(SupabaseClient::new(config));
    let backend = Arc::new(api);

    let login_use_case = Arc::new(LoginUseCase::<Api>::new(backend.clone()));
    let get_patient_programs_use_case =
        Arc::new(GetPatientProgramsUseCase::<Api>::new(backend.clone()));
    let patient_workout_session_use_case =
        Arc::new(PatientWorkoutSessionUseCase::<Api>::new(backend.clone()));
    let submit_patient_workout_feedback_use_case = Arc::new(SubmitPatientWorkoutFeedbackUseCase::<
        Api,
    >::new(backend.clone()));
    let uncomplete_patient_workout_session_use_case = Arc::new(
        UncompletePatientWorkoutSessionUseCase::<Api>::new(backend.clone()),
    );
    let local_notifications = local_notifications_impl();
    let _ = local_notifications.request_permission();

    Ok(AppContext::new(
        backend,
        None,
        login_use_case,
        get_patient_programs_use_case,
        patient_workout_session_use_case,
        submit_patient_workout_feedback_use_case,
        uncomplete_patient_workout_session_use_case,
        local_notifications,
    ))
}
