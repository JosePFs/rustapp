use async_trait::async_trait;

use domain::{credentials::Credentials, error::Result, session::Session};

#[async_trait]
pub trait AuthServiceSend: Send + Sync {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
}
