use std::sync::Arc;

use dioxus::signals::Signal;

use crate::application::use_cases::get_patient_programs::GetPatientProgramsUseCase;
use crate::application::use_cases::login::LoginUseCase;
use crate::application::use_cases::patient_workout_session::PatientWorkoutSessionUseCase;
use crate::application::use_cases::submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackUseCase;
use crate::application::use_cases::uncomplete_patient_workout_session::UncompletePatientWorkoutSessionUseCase;
use crate::application::Backend;
use crate::domain::session::Session;
use crate::infrastructure::supabase::api::Api;

#[derive(Clone)]
pub struct AppContext {
    backend: Arc<dyn Backend>,
    session: Signal<Option<Session>>,
    login_use_case: Arc<LoginUseCase<Api>>,
    get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
    patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
    submit_patient_workout_feedback_use_case: Arc<SubmitPatientWorkoutFeedbackUseCase<Api>>,
    uncomplete_patient_workout_session_use_case:
        Arc<UncompletePatientWorkoutSessionUseCase<Api>>,
}

impl AppContext {
    pub fn new(
        backend: Arc<dyn Backend>,
        session: Option<Session>,
        login_use_case: Arc<LoginUseCase<Api>>,
        get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
        patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
        submit_patient_workout_feedback_use_case: Arc<SubmitPatientWorkoutFeedbackUseCase<Api>>,
        uncomplete_patient_workout_session_use_case:
            Arc<UncompletePatientWorkoutSessionUseCase<Api>>,
    ) -> Self {
        Self {
            backend,
            session: Signal::new(session),
            login_use_case,
            get_patient_programs_use_case,
            patient_workout_session_use_case,
            submit_patient_workout_feedback_use_case,
            uncomplete_patient_workout_session_use_case,
        }
    }

    pub fn backend(&self) -> Arc<dyn Backend> {
        self.backend.clone()
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session.clone()
    }

    pub fn login_use_case(&self) -> Arc<LoginUseCase<Api>> {
        self.login_use_case.clone()
    }

    pub fn get_patient_programs_use_case(&self) -> Arc<GetPatientProgramsUseCase<Api>> {
        self.get_patient_programs_use_case.clone()
    }

    pub fn patient_workout_session_use_case(
        &self,
    ) -> Arc<PatientWorkoutSessionUseCase<Api>> {
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
}
