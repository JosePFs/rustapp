use std::sync::Arc;

use dioxus::signals::Signal;

use crate::application::use_cases::login::LoginUseCase;
use crate::application::Backend;
use crate::domain::session::Session;
use crate::infrastructure::supabase::api::Api;

#[derive(Clone)]
pub struct AppContext {
    backend: Arc<dyn Backend>,
    session: Signal<Option<Session>>,
    login_use_case: Arc<LoginUseCase<Api>>,
}

impl AppContext {
    pub fn new(
        backend: Arc<dyn Backend>,
        session: Option<Session>,
        login_use_case: Arc<LoginUseCase<Api>>,
    ) -> Self {
        Self {
            backend,
            session: Signal::new(session),
            login_use_case,
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
}
