use std::sync::Arc;

use dioxus::signals::Signal;

use crate::application::use_cases::get_patient_programs::GetPatientProgramsUseCase;
use crate::application::use_cases::login::LoginUseCase;
use crate::application::Backend;
use crate::domain::session::Session;
use crate::infrastructure::supabase::api::Api;

#[derive(Clone)]
pub struct AppContext {
    backend: Arc<dyn Backend>,
    session: Signal<Option<Session>>,
    login_use_case: Arc<LoginUseCase<Api>>,
    get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
}

impl AppContext {
    pub fn new(
        backend: Arc<dyn Backend>,
        session: Option<Session>,
        login_use_case: Arc<LoginUseCase<Api>>,
        get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
    ) -> Self {
        Self {
            backend,
            session: Signal::new(session),
            login_use_case,
            get_patient_programs_use_case,
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
}
