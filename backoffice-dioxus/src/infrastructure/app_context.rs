use std::sync::Arc;

use dioxus::signals::Signal;

use crate::application::{
    ports::LocalNotificationService, use_cases::get_patient_programs::GetPatientProgramsUseCase,
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
use crate::domain::session::Session;
use ::infrastructure::supabase::api::Api;

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
    local_notifications: Arc<dyn LocalNotificationService>,
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
        local_notifications: Arc<dyn LocalNotificationService>,
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
            local_notifications,
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

    pub fn local_notifications(&self) -> Arc<dyn LocalNotificationService> {
        self.local_notifications.clone()
    }
}
