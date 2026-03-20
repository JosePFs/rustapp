use std::sync::Arc;

use crate::ports::auth::auth::AuthService;
use crate::ports::auth::{credentials::Credentials, session::Session};
use crate::ports::Backend;
use domain::error::Result;
use domain::vos::role::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginUseCaseArgs {
    pub credentials: Credentials,
}

impl LoginUseCaseArgs {
    pub fn from(email: &str, password: &str) -> Self {
        Self {
            credentials: Credentials::from(email, password),
        }
    }
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

impl From<&Role> for UserProfileType {
    fn from(role: &Role) -> Self {
        match *role {
            Role::Specialist => Self::Specialist,
            Role::Patient => Self::Patient,
        }
    }
}

impl ToString for UserProfileType {
    fn to_string(&self) -> String {
        match self {
            UserProfileType::Specialist => "specialist".to_string(),
            UserProfileType::Patient => "patient".to_string(),
        }
    }
}

impl Default for UserProfileType {
    fn default() -> Self {
        UserProfileType::Patient
    }
}

impl LoginUseCaseResult {
    pub fn is_login_as_patient(&self) -> bool {
        self.user_profile_type == UserProfileType::Patient
    }

    pub fn is_login_as_specialist(&self) -> bool {
        self.user_profile_type == UserProfileType::Specialist
    }
}

pub struct LoginUseCase<B: Backend, A: AuthService> {
    backend: Arc<B>,
    auth: Arc<A>,
}

impl<B: Backend, A: AuthService> LoginUseCase<B, A> {
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
