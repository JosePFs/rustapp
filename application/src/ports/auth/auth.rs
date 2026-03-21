use crate::ports::auth::{credentials::Credentials, session::Session};
use domain::error::Result;

#[common::async_trait_platform]
pub trait AuthService {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
    async fn refresh_session(&self, refresh_token: &str) -> Result<Session>;
}
