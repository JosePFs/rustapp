use async_trait::async_trait;

use crate::domain::credentials::Credentials;
use crate::domain::error::Result;
use crate::domain::session::Session;

#[async_trait(?Send)]
pub trait AuthService {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session>;
}
