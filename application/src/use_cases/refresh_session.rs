use std::sync::Arc;

use crate::ports::auth::auth::AuthService;
use crate::use_cases::login::{login_result_from_session, LoginUseCaseResult};
use domain::error::Result;
use domain::repositories::GetProfilesByIdsRead;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshSessionArgs {
    pub refresh_token: String,
}

impl RefreshSessionArgs {
    pub fn from_refresh_token(refresh_token: impl Into<String>) -> Self {
        Self {
            refresh_token: refresh_token.into(),
        }
    }
}

pub struct RefreshSessionUseCase<R: GetProfilesByIdsRead, A: AuthService> {
    catalog_read: Arc<R>,
    auth: Arc<A>,
}

impl<R: GetProfilesByIdsRead, A: AuthService> RefreshSessionUseCase<R, A> {
    pub fn new(catalog_read: Arc<R>, auth: Arc<A>) -> Self {
        Self { catalog_read, auth }
    }

    pub async fn execute(&self, args: RefreshSessionArgs) -> Result<LoginUseCaseResult> {
        let session = self
            .auth
            .refresh_session(args.refresh_token.as_str())
            .await?;
        login_result_from_session(&*self.catalog_read, session).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::ports::auth::session::Session;
    use crate::test_mocks::{FakeAuthService, FakeGetProfilesByIds};
    use crate::use_cases::login::UserProfileType;
    use domain::error::DomainError;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;

    #[tokio::test]
    async fn refresh_session_propagates_auth_error() {
        let auth = FakeAuthService::new().with_refresh_err(DomainError::SessionNotFound);
        let catalog = FakeGetProfilesByIds::new_ok(vec![]);
        let uc = RefreshSessionUseCase::new(Arc::new(catalog), Arc::new(auth));

        let err = uc
            .execute(RefreshSessionArgs::from_refresh_token("rt"))
            .await
            .unwrap_err();

        assert_eq!(err, DomainError::SessionNotFound);
    }

    #[tokio::test]
    async fn refresh_session_maps_specialist_profile() {
        let uid = "550e8400-e29b-41d4-a716-446655440030";
        let session = Session::new("at".into(), Some("rt".into()), uid.to_string());
        let auth = FakeAuthService::new().with_refresh_ok(session);
        let id = Id::try_from(uid).unwrap();
        let email = Email::try_from("s@example.com").unwrap();
        let full_name = FullName::try_from("Spec").unwrap();
        let role = Role::try_from("specialist").unwrap();
        let profile = Profile::new(id, email, full_name, role);
        let catalog = FakeGetProfilesByIds::new_ok(vec![profile]);
        let uc = RefreshSessionUseCase::new(Arc::new(catalog), Arc::new(auth));

        let res = uc
            .execute(RefreshSessionArgs::from_refresh_token("rt"))
            .await
            .unwrap();

        assert_eq!(res.user_profile_type, UserProfileType::Specialist);
    }
}
