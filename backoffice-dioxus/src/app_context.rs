use std::sync::Arc;

use dioxus::signals::Signal;

use application::{
    use_cases::get_patient_programs::GetPatientProgramsUseCase,
    use_cases::get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesUseCase,
    use_cases::list_exercise_library::ListExerciseLibraryUseCase,
    use_cases::list_workout_library::ListWorkoutLibraryUseCase, use_cases::login::LoginUseCase,
    use_cases::patient_progress::PatientProgressUseCase,
    use_cases::patient_workout_session::PatientWorkoutSessionUseCase,
    use_cases::specialist_programs_data::SpecialistProgramsDataUseCase,
    use_cases::submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackUseCase,
    use_cases::uncomplete_patient_workout_session::UncompletePatientWorkoutSessionUseCase,
    use_cases::workout_editor_data::WorkoutEditorDataUseCase, Backend,
};
use domain::{error::Result, session::Session};
use infrastructure::supabase::{api::Api, client::SupabaseClient, config::SupabaseConfig};

#[derive(Clone)]
pub struct AppContext {
    backend: Arc<dyn Backend>,
    session: Signal<Option<Session>>,
    login_use_case: Arc<LoginUseCase<Api>>,
    get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
    get_specialist_patients_with_profiles_use_case:
        Arc<GetSpecialistPatientsWithProfilesUseCase<Api>>,
    specialist_programs_data_use_case: Arc<SpecialistProgramsDataUseCase<Api>>,
    list_exercise_library_use_case: Arc<ListExerciseLibraryUseCase<Api>>,
    list_workout_library_use_case: Arc<ListWorkoutLibraryUseCase<Api>>,
    patient_progress_use_case: Arc<PatientProgressUseCase<Api>>,
    patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
    submit_patient_workout_feedback_use_case: Arc<SubmitPatientWorkoutFeedbackUseCase<Api>>,
    uncomplete_patient_workout_session_use_case: Arc<UncompletePatientWorkoutSessionUseCase<Api>>,
    workout_editor_data_use_case: Arc<WorkoutEditorDataUseCase<Api>>,
}

impl AppContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        backend: Arc<dyn Backend>,
        session: Option<Session>,
        login_use_case: Arc<LoginUseCase<Api>>,
        get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
        get_specialist_patients_with_profiles_use_case: Arc<
            GetSpecialistPatientsWithProfilesUseCase<Api>,
        >,
        specialist_programs_data_use_case: Arc<SpecialistProgramsDataUseCase<Api>>,
        list_exercise_library_use_case: Arc<ListExerciseLibraryUseCase<Api>>,
        list_workout_library_use_case: Arc<ListWorkoutLibraryUseCase<Api>>,
        patient_progress_use_case: Arc<PatientProgressUseCase<Api>>,
        patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
        submit_patient_workout_feedback_use_case: Arc<SubmitPatientWorkoutFeedbackUseCase<Api>>,
        uncomplete_patient_workout_session_use_case: Arc<
            UncompletePatientWorkoutSessionUseCase<Api>,
        >,
        workout_editor_data_use_case: Arc<WorkoutEditorDataUseCase<Api>>,
    ) -> Self {
        Self {
            backend,
            session: Signal::new(session),
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
        }
    }

    pub fn backend(&self) -> Arc<dyn Backend> {
        self.backend.clone()
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session
    }

    pub fn login_use_case(&self) -> Arc<LoginUseCase<Api>> {
        self.login_use_case.clone()
    }

    pub fn get_patient_programs_use_case(&self) -> Arc<GetPatientProgramsUseCase<Api>> {
        self.get_patient_programs_use_case.clone()
    }

    pub fn get_specialist_patients_with_profiles_use_case(
        &self,
    ) -> Arc<GetSpecialistPatientsWithProfilesUseCase<Api>> {
        self.get_specialist_patients_with_profiles_use_case.clone()
    }

    pub fn specialist_programs_data_use_case(&self) -> Arc<SpecialistProgramsDataUseCase<Api>> {
        self.specialist_programs_data_use_case.clone()
    }

    pub fn list_exercise_library_use_case(&self) -> Arc<ListExerciseLibraryUseCase<Api>> {
        self.list_exercise_library_use_case.clone()
    }

    pub fn list_workout_library_use_case(&self) -> Arc<ListWorkoutLibraryUseCase<Api>> {
        self.list_workout_library_use_case.clone()
    }

    pub fn patient_progress_use_case(&self) -> Arc<PatientProgressUseCase<Api>> {
        self.patient_progress_use_case.clone()
    }

    pub fn patient_workout_session_use_case(&self) -> Arc<PatientWorkoutSessionUseCase<Api>> {
        self.patient_workout_session_use_case.clone()
    }

    pub fn submit_patient_workout_feedback_use_case(
        &self,
    ) -> Arc<SubmitPatientWorkoutFeedbackUseCase<Api>> {
        self.submit_patient_workout_feedback_use_case.clone()
    }

    pub fn uncomplete_patient_workout_session_use_case(
        &self,
    ) -> Arc<UncompletePatientWorkoutSessionUseCase<Api>> {
        self.uncomplete_patient_workout_session_use_case.clone()
    }

    pub fn workout_editor_data_use_case(&self) -> Arc<WorkoutEditorDataUseCase<Api>> {
        self.workout_editor_data_use_case.clone()
    }
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
    ))
}
