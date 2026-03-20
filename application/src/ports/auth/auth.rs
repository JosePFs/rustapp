use async_trait::async_trait;

use crate::ports::auth::{credentials::Credentials, session::Session};
use domain::error::Result;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AuthService {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
    async fn refresh_session(&self, refresh_token: &str) -> Result<Session>;
}
