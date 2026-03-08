use dioxus::{prelude::*, CapturedError};

use crate::domain::{credentials::Credentials, profile::Profile, session::Session};
use crate::infrastructure::ui::hooks::app_context::use_app_context;

pub fn use_login() -> (
    Action<(Credentials,), Session>,
    Resource<Option<Vec<Profile>>>,
) {
    let app_context = use_app_context();
    let backend = app_context.backend();
    let mut app_session = app_context.session();

    let backend_for_action = backend.clone();
    let login_action = use_action(move |credentials: Credentials| {
        let backend = backend_for_action.clone();

        async move {
            backend
                .sign_in(&credentials)
                .await
                .map_err(Into::<CapturedError>::into)
        }
    });

    let backend_for_profile_loader = backend.clone();
    let profile_loader = use_resource(move || {
        let backend = backend_for_profile_loader.clone();
        let login_action = login_action.value();

        async move {
            let Some(Ok(session)) = login_action else {
                return None;
            };

            let session = session();
            let profiles = backend
                .get_profiles_by_ids(&[session.user_id().to_string()], session.access_token())
                .await
                .ok();

            profiles
        }
    });

    use_effect(move || {
        if profile_loader.read().is_some() {
            if let Some(Ok(session)) = login_action.value() {
                app_session.set(Some(session()));
            }
        }
    });

    (login_action, profile_loader)
}
