use std::sync::Arc;

use async_trait::async_trait;

use super::client::SupabaseClient;
use crate::supabase::config::SupabaseConfig;
use crate::supabase::DEFAULT_CLIENT;
use application::ports::auth::auth::AuthService;
use application::ports::auth::{credentials::Credentials, session::Session};
use domain::error::{DomainError, Result};

#[derive(Clone)]
pub struct SupabaseAuth {
    client: Arc<SupabaseClient>,
}

impl SupabaseAuth {
    fn new(client: Arc<SupabaseClient>) -> Self {
        Self { client }
    }

    pub fn builder() -> SupabaseAuthBuilder {
        SupabaseAuthBuilder::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AuthService for SupabaseAuth {
    async fn sign_in(&self, credentials: &Credentials) -> domain::error::Result<Session> {
        self.client
            .sign_in(credentials)
            .await
            .map_err(|e| {
                log::warn!("Login failed: {}", e);
                DomainError::Login("wrong_credentials".to_string())
            })
            .map(|auth| Session::new(auth.access_token, auth.refresh_token, auth.user.id))
    }

    async fn refresh_session(&self, refresh_token: &str) -> Result<Session> {
        self.client
            .refresh_session(refresh_token)
            .await
            .map_err(|e| {
                log::warn!("Refresh session failed: {}", e);
                DomainError::Login("refresh_token_expired".to_string())
            })
            .map(|auth| Session::new(auth.access_token, auth.refresh_token, auth.user.id))
    }
}

pub struct SupabaseAuthBuilder {
    config: Option<SupabaseConfig>,
    client: Option<Arc<SupabaseClient>>,
}

impl SupabaseAuthBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            client: None,
        }
    }

    pub fn with_config(mut self, config: SupabaseConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_client(mut self, client: Arc<SupabaseClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> SupabaseAuth {
        let client = self.client.unwrap_or_else(|| DEFAULT_CLIENT.clone());
        SupabaseAuth::new(client)
    }
}
