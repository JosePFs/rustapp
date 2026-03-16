use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use crate::hooks::AsyncState;
use application::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult};
use domain::credentials::Credentials;

#[derive(Debug, Clone)]
pub struct UseLogin {
    pub action: Action<(Credentials,), LoginUseCaseResult>,
    pub state: Signal<AsyncState<LoginUseCaseResult>>,
}

pub fn use_login() -> UseLogin {
    let app_context = use_app_context();
    let login_use_case = app_context.login_use_case();
    let mut app_session = app_context.session();
    let mut state = use_signal(|| AsyncState::<LoginUseCaseResult>::Idle);

    let login_use_case_for_action = login_use_case.clone();
    let action = use_action(move |credentials: Credentials| {
        let login_use_case = login_use_case_for_action.clone();

        async move {
            state.set(AsyncState::<LoginUseCaseResult>::Loading);

            login_use_case
                .execute(LoginUseCaseArgs { credentials })
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
