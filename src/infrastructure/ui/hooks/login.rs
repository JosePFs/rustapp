use dioxus::prelude::*;

use crate::application::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult, UserProfileType};
use crate::domain::credentials::Credentials;
use crate::infrastructure::ui::hooks::app_context::use_app_context;

#[derive(Debug, Clone, PartialEq)]
pub enum UseLoginState {
    None,
    Pending,
    Success(UserProfileType),
    Error(String),
}

impl UseLoginState {
    pub fn is_pending(&self) -> bool {
        matches!(self, UseLoginState::Pending)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, UseLoginState::Error(_))
    }

    pub fn error(&self) -> String {
        match self {
            UseLoginState::Error(error) => error.clone(),
            _ => String::new(),
        }
    }

    pub fn is_login_as_patient(&self) -> bool {
        matches!(self, UseLoginState::Success(UserProfileType::Patient))
    }

    pub fn is_login_as_specialist(&self) -> bool {
        matches!(self, UseLoginState::Success(UserProfileType::Specialist))
    }
}

#[derive(Debug, Clone)]
pub struct UseLogin {
    pub action: Action<(Credentials,), LoginUseCaseResult>,
    pub state: Signal<UseLoginState>,
}

pub fn use_login() -> UseLogin {
    let app_context = use_app_context();
    let login_use_case = app_context.login_use_case();
    let mut app_session = app_context.session();
    let mut state = use_signal(|| UseLoginState::None);

    let login_use_case_for_action = login_use_case.clone();
    let action = use_action(move |credentials: Credentials| {
        let login_use_case = login_use_case_for_action.clone();

        state.set(UseLoginState::Pending);
        async move {
            login_use_case
                .execute(LoginUseCaseArgs { credentials })
                .await
                .map(|login_use_case_result| {
                    state.set(UseLoginState::Success(
                        login_use_case_result.user_profile_type.clone(),
                    ));
                    login_use_case_result
                })
                .map_err(|e| {
                    state.set(UseLoginState::Error(e.to_string()));
                    e
                })
        }
    });

    use_effect(move || {
        if let Some(Ok(login_use_case_result)) = action.value() {
            app_session.set(Some(login_use_case_result().session));
        }
    });

    UseLogin { action, state }
}
