use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use crate::hooks::AsyncState;
use application::ports::backoffice_api::{LoginArgs, LoginResult};

#[derive(Debug, Clone)]
pub struct UseLogin {
    pub action: Action<((String, String),), LoginResult>,
    pub state: Signal<AsyncState<LoginResult>>,
}

pub fn use_login() -> UseLogin {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<LoginResult>::Idle);

    let facade_for_action = facade.clone();
    let action = use_action(move |(email, password): (String, String)| {
        let facade = facade_for_action.clone();

        async move {
            state.set(AsyncState::<LoginResult>::Loading);

            let args = LoginArgs {
                email,
                password,
            };
            match facade.login(args).await {
                Ok(login_result) => {
                    state.set(AsyncState::<LoginResult>::Ready(login_result.clone()));
                    Ok(login_result)
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    Err(e)
                }
            }
        }
    });

    UseLogin { action, state }
}
