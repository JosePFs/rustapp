use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use crate::hooks::AsyncState;
use application::ports::auth::credentials::Credentials;
use application::ports::BackofficeApi;
use application::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult};

#[derive(Debug, Clone)]
pub struct UseLogin {
    pub action: Action<((String, String),), LoginUseCaseResult>,
    pub state: Signal<AsyncState<LoginUseCaseResult>>,
}

pub fn use_login() -> UseLogin {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut app_session = app_context.session();
    let mut state = use_signal(|| AsyncState::<LoginUseCaseResult>::Idle);

    let facade_for_action = facade.clone();
    let action = use_action(move |(email, password): (String, String)| {
        let facade = facade_for_action.clone();

        async move {
            state.set(AsyncState::<LoginUseCaseResult>::Loading);

            facade
                .login(LoginUseCaseArgs {
                    credentials: Credentials::from(&email, &password),
                })
                .await
                .map(|login_use_case_result| {
                    state.set(AsyncState::<LoginUseCaseResult>::Ready(
                        login_use_case_result.clone(),
                    ));
                    login_use_case_result
                })
                .map_err(|e| {
                    state.set(AsyncState::<LoginUseCaseResult>::Error(e.clone()));
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
