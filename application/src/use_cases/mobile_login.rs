use std::sync::Arc;

use crate::ports::auth::AuthService;
use crate::ports::error::{ApplicationError, Result};
use crate::use_cases::login::{login_result_from_session, LoginUseCaseArgs, LoginUseCaseResult};
use domain::repositories::GetProfilesByIdsRead;

pub struct MobileLoginUseCase<R: GetProfilesByIdsRead, A: AuthService> {
    catalog_read: Arc<R>,
    auth: Arc<A>,
}

impl<R: GetProfilesByIdsRead, A: AuthService> MobileLoginUseCase<R, A> {
    pub fn new(catalog_read: Arc<R>, auth: Arc<A>) -> Self {
        Self { catalog_read, auth }
    }

    pub async fn execute(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        let session = self.auth.sign_in(&args.credentials).await.map_err(ApplicationError::from)?;
        login_result_from_session(&*self.catalog_read, session).await.map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use crate::ports::auth::{AuthService, Credentials, Session};
    use crate::use_cases::login::LoginUseCaseArgs;
    use domain::error::{DomainError, Result};
    use domain::repositories::GetProfilesByIdsRead;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;

    #[tokio::test]
    async fn mobile_login_uses_same_session_mapping_as_login() {
        let uid = "550e8400-e29b-41d4-a716-446655440040";
        let session = Session::new("at".into(), None, uid.to_string(), None);
        let auth = MockAuthService::new().with_sign_in_ok(session);
        let id = Id::try_from(uid).unwrap();
        let email = Email::try_from("m@example.com").unwrap();
        let full_name = FullName::try_from("Mob").unwrap();
        let role = Role::try_from("patient").unwrap();
        let profile = Profile::new(id, email, full_name, role);
        let catalog = MockGetProfilesByIdsRead::new_ok(vec![profile]);
        let uc = MobileLoginUseCase::new(Arc::new(catalog), Arc::new(auth));

        let res = uc
            .execute(LoginUseCaseArgs::from("m@example.com", "pw"))
            .await
            .unwrap();

        assert!(res.is_login_as_patient());
    }

    #[derive(Clone, Default)]
    struct MockAuthService {
        inner: Arc<Mutex<MockAuthServiceState>>,
    }

    #[derive(Default)]
    struct MockAuthServiceState {
        sign_in_result: Option<Result<Session>>,
    }

    impl MockAuthService {
        fn new() -> Self {
            Self::default()
        }

        fn with_sign_in_ok(self, session: Session) -> Self {
            self.inner.lock().unwrap().sign_in_result = Some(Ok(session));
            self
        }
    }

    #[common::async_trait_platform]
    impl AuthService for MockAuthService {
        async fn sign_in(&self, _credentials: &Credentials) -> Result<Session> {
            self.inner
                .lock()
                .unwrap()
                .sign_in_result
                .clone()
                .unwrap_or_else(|| Err(DomainError::Login("fake: sign_in not configured".into())))
        }

        async fn refresh_session(&self, _refresh_token: &str) -> Result<Session> {
            Err(DomainError::Login(
                "fake: refresh_session not used by mobile_login tests".into(),
            ))
        }

        fn get_session(&self) -> Option<Session> {
            self.inner
                .lock()
                .unwrap()
                .sign_in_result
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
