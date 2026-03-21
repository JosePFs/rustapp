use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::ports::auth::auth::AuthService;
use crate::ports::auth::{credentials::Credentials, session::Session};
use domain::error::{DomainError, Result};

#[derive(Clone, Default)]
pub struct FakeAuthService {
    inner: Arc<Mutex<FakeAuthState>>,
}

#[derive(Default)]
struct FakeAuthState {
    sign_in_result: Option<Result<Session>>,
    refresh_result: Option<Result<Session>>,
}

impl FakeAuthService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sign_in_ok(self, session: Session) -> Self {
        self.inner.lock().unwrap().sign_in_result = Some(Ok(session));
        self
    }

    pub fn with_sign_in_err(self, err: DomainError) -> Self {
        self.inner.lock().unwrap().sign_in_result = Some(Err(err));
        self
    }

    pub fn with_refresh_ok(self, session: Session) -> Self {
        self.inner.lock().unwrap().refresh_result = Some(Ok(session));
        self
    }

    pub fn with_refresh_err(self, err: DomainError) -> Self {
        self.inner.lock().unwrap().refresh_result = Some(Err(err));
        self
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AuthService for FakeAuthService {
    async fn sign_in(&self, _credentials: &Credentials) -> Result<Session> {
        self.inner
            .lock()
            .unwrap()
            .sign_in_result
            .clone()
            .unwrap_or_else(|| Err(DomainError::Login("fake: sign_in not configured".into())))
    }

    async fn refresh_session(&self, _refresh_token: &str) -> Result<Session> {
        self.inner
            .lock()
            .unwrap()
            .refresh_result
            .clone()
            .unwrap_or_else(|| {
                Err(DomainError::Login(
                    "fake: refresh_session not configured".into(),
                ))
            })
    }
}
