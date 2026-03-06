use std::sync::Arc;

use dioxus::signals::Signal;

use crate::application::ports::Backend;
use crate::domain::session::Session;

#[derive(Clone)]
pub struct AppContext {
    backend: Arc<dyn Backend>,
    session: Signal<Option<Session>>,
}

impl AppContext {
    pub fn new(backend: Arc<dyn Backend>, session: Option<Session>) -> Self {
        Self {
            backend,
            session: Signal::new(session),
        }
    }

    pub fn backend(&self) -> Arc<dyn Backend> {
        self.backend.clone()
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session.clone()
    }
}
