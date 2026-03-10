use std::sync::Arc;

use crate::application::Backend;
use crate::domain::credentials::Credentials;
use crate::domain::error::Result;
use crate::domain::role::Role;
use crate::domain::session::Session;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginUseCaseArgs {
    pub credentials: Credentials,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginUseCaseResult {
    pub session: Session,
    pub user_profile_type: UserProfileType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserProfileType {
    Specialist,
    Patient,
}

pub struct LoginUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> LoginUseCase<B> {
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
            .map(|role| match role {
                Role::Specialist => UserProfileType::Specialist,
                Role::Patient => UserProfileType::Patient,
            })
            .unwrap_or(UserProfileType::Patient);

        Ok(LoginUseCaseResult {
            session,
            user_profile_type,
        })
    }
}
