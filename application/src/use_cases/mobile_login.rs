use std::sync::Arc;

use crate::ports::MobileBackend;
use crate::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult, UserProfileType};
use domain::error::Result;

pub struct MobileLoginUseCase<B: MobileBackend> {
    backend: Arc<B>,
}

impl<B: MobileBackend> MobileLoginUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        let session = self.backend.sign_in(&args.credentials).await?;

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
