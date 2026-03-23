use std::sync::Arc;

use crate::ports::auth::AuthService;
use crate::ports::error::{ApplicationError, Result};
use crate::use_cases::login::{login_result_from_session, LoginUseCaseResult};
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
            .await
            .map_err(ApplicationError::from)?;
        login_result_from_session(&*self.catalog_read, session)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use crate::ports::auth::{AuthService, Credentials, Session};
    use crate::use_cases::login::UserProfileType;
    use domain::error::DomainError;
    use domain::error::Result;
    use domain::repositories::GetProfilesByIdsRead;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;

    #[tokio::test]
    async fn refresh_session_propagates_auth_error() {
        let auth = MockAuthService::new().with_refresh_err(DomainError::AuthenticationFailed(
            "Session not found".to_string(),
        ));
        let catalog = MockGetProfilesByIdsRead::new_ok(vec![]);
        let uc = RefreshSessionUseCase::new(Arc::new(catalog), Arc::new(auth));

        let err = uc
            .execute(RefreshSessionArgs::from_refresh_token("rt"))
            .await
            .unwrap_err();

        assert_eq!(
            err,
            ApplicationError::DomainError(DomainError::AuthenticationFailed(
                "Session not found".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn refresh_session_maps_specialist_profile() {
        let uid = "550e8400-e29b-41d4-a716-446655440030";
        let session = Session::new("at".into(), Some("rt".into()), uid.to_string(), None);
        let auth = MockAuthService::new().with_refresh_ok(session);
        let id = Id::try_from(uid).unwrap();
        let email = Email::try_from("s@example.com").unwrap();
        let full_name = FullName::try_from("Spec").unwrap();
        let role = Role::try_from("specialist").unwrap();
        let profile = Profile::new(id, email, full_name, role);
        let catalog = MockGetProfilesByIdsRead::new_ok(vec![profile]);
        let uc = RefreshSessionUseCase::new(Arc::new(catalog), Arc::new(auth));

        let res = uc
            .execute(RefreshSessionArgs::from_refresh_token("rt"))
            .await
            .unwrap();

        assert_eq!(res.user_profile_type, UserProfileType::Specialist);
    }

    #[derive(Clone, Default)]
    struct MockAuthService {
        inner: Arc<Mutex<MockAuthServiceState>>,
    }

    #[derive(Default)]
    struct MockAuthServiceState {
        refresh_result: Option<Result<Session>>,
    }

    impl MockAuthService {
        fn new() -> Self {
            Self::default()
        }

        fn with_refresh_ok(self, session: Session) -> Self {
            self.inner.lock().unwrap().refresh_result = Some(Ok(session));
            self
        }

        fn with_refresh_err(self, err: DomainError) -> Self {
            self.inner.lock().unwrap().refresh_result = Some(Err(err));
            self
        }
    }

    #[common::async_trait_platform]
    impl AuthService for MockAuthService {
        async fn sign_in(&self, _credentials: &Credentials) -> Result<Session> {
            Err(DomainError::Login(
                "fake: sign_in not used by refresh_session tests".into(),
            ))
        }

        async fn refresh_session(&self, _refresh_token: &str) -> Result<Session> {
            self.inner
                .lock()
                .unwrap()
                .refresh_result
                .clone()
                .unwrap_or_else(|| {
                    Err(DomainError::Login(
                        "fake: refresh_session not configured".into(),
                    ))
                })
        }

        fn get_session(&self) -> Option<Session> {
            self.inner
                .lock()
                .unwrap()
                .refresh_result
                .clone()
                .map(|r| r.unwrap())
        }
    }

    #[derive(Clone)]
    struct MockGetProfilesByIdsRead {
        profiles: Arc<Mutex<Result<Vec<Profile>>>>,
    }

    impl MockGetProfilesByIdsRead {
        fn new_ok(profiles: Vec<Profile>) -> Self {
            Self {
                profiles: Arc::new(Mutex::new(Ok(profiles))),
            }
        }
    }

    #[common::async_trait_platform]
    impl GetProfilesByIdsRead for MockGetProfilesByIdsRead {
        async fn get_profiles_by_ids(&self, _ids: &[Id]) -> Result<Vec<Profile>> {
            self.profiles.lock().unwrap().clone()
        }
    }
}
