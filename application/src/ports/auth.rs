use async_trait::async_trait;

use domain::{credentials::Credentials, error::Result, session::Session};

#[async_trait(?Send)]
pub trait AuthService {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
    async fn refresh_session(&self, refresh_token: &str) -> Result<Session>;
}
