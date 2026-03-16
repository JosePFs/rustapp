use std::sync::Arc;

use crate::application::ports::{LocalNotificationService, StubLocalNotificationService};
use crate::application::use_cases::{
    get_patient_programs::GetPatientProgramsUseCase,
    get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesUseCase,
    list_exercise_library::ListExerciseLibraryUseCase,
    list_workout_library::ListWorkoutLibraryUseCase, login::LoginUseCase,
    patient_progress::PatientProgressUseCase,
    patient_workout_session::PatientWorkoutSessionUseCase,
    specialist_programs_data::SpecialistProgramsDataUseCase,
    submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackUseCase,
    uncomplete_patient_workout_session::UncompletePatientWorkoutSessionUseCase,
    workout_editor_data::WorkoutEditorDataUseCase,
};
use crate::domain::error::Result;
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::supabase::{api::Api, client::SupabaseClient, config::SupabaseConfig};

fn local_notifications_impl() -> Arc<dyn LocalNotificationService> {
    Arc::new(StubLocalNotificationService::default())
}

pub fn build_app_context() -> Result<AppContext> {
    let config = SupabaseConfig::from_env()?;

    let api = Api::new(SupabaseClient::new(config));
    let backend = Arc::new(api);

    let login_use_case = Arc::new(LoginUseCase::<Api>::new(backend.clone()));
    let get_patient_programs_use_case =
        Arc::new(GetPatientProgramsUseCase::<Api>::new(backend.clone()));
    let get_specialist_patients_with_profiles_use_case = Arc::new(
        GetSpecialistPatientsWithProfilesUseCase::<Api>::new(backend.clone()),
    );
    let specialist_programs_data_use_case =
        Arc::new(SpecialistProgramsDataUseCase::<Api>::new(backend.clone()));
    let list_exercise_library_use_case =
        Arc::new(ListExerciseLibraryUseCase::<Api>::new(backend.clone()));
    let list_workout_library_use_case =
        Arc::new(ListWorkoutLibraryUseCase::<Api>::new(backend.clone()));
    let patient_progress_use_case = Arc::new(PatientProgressUseCase::<Api>::new(backend.clone()));
    let patient_workout_session_use_case =
        Arc::new(PatientWorkoutSessionUseCase::<Api>::new(backend.clone()));
    let submit_patient_workout_feedback_use_case = Arc::new(SubmitPatientWorkoutFeedbackUseCase::<
        Api,
    >::new(backend.clone()));
    let uncomplete_patient_workout_session_use_case = Arc::new(
        UncompletePatientWorkoutSessionUseCase::<Api>::new(backend.clone()),
    );
    let workout_editor_data_use_case =
        Arc::new(WorkoutEditorDataUseCase::<Api>::new(backend.clone()));
    let local_notifications = local_notifications_impl();
    let _ = local_notifications.request_permission();

    Ok(AppContext::new(
        backend,
        None,
        login_use_case,
        get_patient_programs_use_case,
        get_specialist_patients_with_profiles_use_case,
        specialist_programs_data_use_case,
        list_exercise_library_use_case,
        list_workout_library_use_case,
        patient_progress_use_case,
        patient_workout_session_use_case,
        submit_patient_workout_feedback_use_case,
        uncomplete_patient_workout_session_use_case,
        workout_editor_data_use_case,
        local_notifications,
    ))
}
