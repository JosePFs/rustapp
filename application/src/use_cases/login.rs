use std::sync::Arc;

use crate::ports::auth::auth::AuthService;
use crate::ports::auth::{credentials::Credentials, session::Session};
use domain::error::Result;
use domain::repositories::GetProfilesByIdsRead;
use domain::vos::id::Id;
use domain::vos::role::Role;
use domain::vos::AccessToken;

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

pub struct LoginUseCase<R: GetProfilesByIdsRead, A: AuthService> {
    catalog_read: Arc<R>,
    auth: Arc<A>,
}

impl<R: GetProfilesByIdsRead, A: AuthService> LoginUseCase<R, A> {
    pub fn new(catalog_read: Arc<R>, auth: Arc<A>) -> Self {
        Self { catalog_read, auth }
    }

    pub async fn execute(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        let session = self.auth.sign_in(&args.credentials).await?;
        login_result_from_session(&*self.catalog_read, session).await
    }
}

pub(crate) async fn login_result_from_session<R>(
    catalog_read: &R,
    session: Session,
) -> Result<LoginUseCaseResult>
where
    R: GetProfilesByIdsRead,
{
    let user_id = Id::try_from(session.user_id().to_string())?;
    let access = AccessToken::try_from(session.access_token().to_string())?;
    let profiles = catalog_read
        .get_profiles_by_ids(&[user_id], &access)
        .await
        .ok();

    let user_profile_type = profiles
        .and_then(|profiles| profiles.into_iter().next())
        .map(|p| UserProfileType::from(p.role()))
        .unwrap_or_default();

    Ok(LoginUseCaseResult {
        session,
        user_profile_type,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ports::auth::auth::AuthService;
    use crate::ports::auth::{credentials::Credentials, session::Session};
    use domain::error::{DomainError, Result};
    use domain::repositories::GetProfilesByIdsRead;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;
    use domain::vos::AccessToken;

    #[tokio::test]
    async fn login_propagates_auth_error() {
        let auth = MockAuthService::new().with_sign_in_err(DomainError::Login("bad".into()));
        let catalog = MockGetProfilesByIdsRead::new_ok(vec![]);
        let uc = LoginUseCase::new(Arc::new(catalog), Arc::new(auth));

        let err = uc
            .execute(LoginUseCaseArgs::from("a@b.com", "pw"))
            .await
            .unwrap_err();

        assert_eq!(err, DomainError::Login("bad".into()));
    }

    #[tokio::test]
    async fn login_patient_profile_from_catalog() {
        let uid = "550e8400-e29b-41d4-a716-446655440020";
        let session = Session::new("at".into(), None, uid.to_string());
        let auth = MockAuthService::new().with_sign_in_ok(session);
        let catalog = MockGetProfilesByIdsRead::new_ok(vec![patient_profile(uid)]);
        let uc = LoginUseCase::new(Arc::new(catalog), Arc::new(auth));

        let res = uc
            .execute(LoginUseCaseArgs::from("a@b.com", "pw"))
            .await
            .unwrap();

        assert!(res.is_login_as_patient());
    }

    fn patient_profile(user_id: &str) -> Profile {
        let id = Id::try_from(user_id).unwrap();
        let email = Email::try_from("u@example.com").unwrap();
        let full_name = FullName::try_from("Test User").unwrap();
        let role = Role::try_from("patient").unwrap();
        Profile::new(id, email, full_name, role)
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

        fn with_sign_in_err(self, err: DomainError) -> Self {
            self.inner.lock().unwrap().sign_in_result = Some(Err(err));
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
                "fake: refresh_session not used by login tests".into(),
            ))
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
        async fn get_profiles_by_ids(
            &self,
            _ids: &[Id],
            _access_token: &AccessToken,
        ) -> Result<Vec<Profile>> {
            self.profiles.lock().unwrap().clone()
        }
    }
}
