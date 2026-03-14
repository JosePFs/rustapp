use async_trait::async_trait;

use crate::domain::{credentials::Credentials, error::Result, session::Session};

#[async_trait(?Send)]
pub trait AuthService {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
}
