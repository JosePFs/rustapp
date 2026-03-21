use std::sync::Arc;

use crate::ports::auth::auth::AuthService;
use crate::use_cases::login::{login_result_from_session, LoginUseCaseArgs, LoginUseCaseResult};
use domain::error::Result;
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
        let session = self.auth.sign_in(&args.credentials).await?;
        login_result_from_session(&*self.catalog_read, session).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::ports::auth::session::Session;
    use crate::test_mocks::{FakeAuthService, FakeGetProfilesByIds};
    use crate::use_cases::login::LoginUseCaseArgs;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::profile::Profile;
    use domain::vos::role::Role;

    #[tokio::test]
    async fn mobile_login_uses_same_session_mapping_as_login() {
        let uid = "550e8400-e29b-41d4-a716-446655440040";
        let session = Session::new("at".into(), None, uid.to_string());
        let auth = FakeAuthService::new().with_sign_in_ok(session);
        let id = Id::try_from(uid).unwrap();
        let email = Email::try_from("m@example.com").unwrap();
        let full_name = FullName::try_from("Mob").unwrap();
        let role = Role::try_from("patient").unwrap();
        let profile = Profile::new(id, email, full_name, role);
        let catalog = FakeGetProfilesByIds::new_ok(vec![profile]);
        let uc = MobileLoginUseCase::new(Arc::new(catalog), Arc::new(auth));

        let res = uc
            .execute(LoginUseCaseArgs::from("m@example.com", "pw"))
            .await
            .unwrap();

        assert!(res.is_login_as_patient());
    }
}
