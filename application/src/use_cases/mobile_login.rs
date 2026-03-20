use std::sync::Arc;

use crate::ports::auth::auth::AuthService;
use crate::ports::MobileBackend;
use crate::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult, UserProfileType};
use domain::error::Result;

pub struct MobileLoginUseCase<B: MobileBackend, A: AuthService> {
    backend: Arc<B>,
    auth: Arc<A>,
}

impl<B: MobileBackend, A: AuthService> MobileLoginUseCase<B, A> {
    pub fn new(backend: Arc<B>, auth: Arc<A>) -> Self {
        Self { backend, auth }
    }

    pub async fn execute(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        let session = self.auth.sign_in(&args.credentials).await?;

        let profiles = self
            .backend
            .get_profiles_by_ids(&[session.user_id().to_string()], session.access_token())
            .await
            .ok();

        let user_profile_type = profiles
            .map(|profiles| profiles.into_iter().next().map(|p| p.role().clone()))
            .flatten()
            .map(|role| UserProfileType::from(&role))
            .unwrap_or_default();

        Ok(LoginUseCaseResult {
            session,
            user_profile_type,
        })
    }
}
