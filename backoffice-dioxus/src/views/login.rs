use application::use_cases::login::LoginUseCaseResult;
use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::components::{Login, LoginResult};
use crate::hooks::{login::use_login, AsyncState};
use crate::Route;

#[component]
pub fn LoginView() -> Element {
    let nav = use_navigator();
    let mut login = use_login();
    let login_result: Memo<LoginResult> = use_memo(move || login.state.read().clone().into());

    use_effect(move || {
        if login_result.read().is_success() {
            nav.push(Route::SpecialistPatients {});
        }
    });

    rsx! {
        Login {
            background_image: asset!("/assets/login.webp"),
            onsubmit: move |(email, password): (String, String)| {
                login.action.call((email, password));
            },
            login_result,
        }
    }
}

impl From<LoginUseCaseResult> for LoginResult {
    fn from(login_use_case_result: LoginUseCaseResult) -> Self {
        if login_use_case_result.is_login_as_specialist() {
            LoginResult::Success
        } else {
            LoginResult::Error(t!("wrong_credentials"))
        }
    }
}

impl From<AsyncState<LoginUseCaseResult>> for LoginResult {
    fn from(state: AsyncState<LoginUseCaseResult>) -> Self {
        if state.is_idle() {
            LoginResult::Idle
        } else if state.is_loading() {
            LoginResult::Pending
        } else {
            state
                .data()
                .map(|login_use_case_result| login_use_case_result.clone().into())
                .unwrap_or(LoginResult::Error(t!("wrong_credentials")))
        }
    }
}
