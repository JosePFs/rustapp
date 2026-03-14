use dioxus::prelude::*;
use dioxus_router::use_navigator;

use crate::domain::credentials::Credentials;
use crate::infrastructure::ui::components::{Login, LoginResult};
use crate::infrastructure::ui::hooks::login::use_login;
use crate::Route;

#[component]
pub fn LoginView() -> Element {
    let nav = use_navigator();
    let mut login = use_login();

    use_effect(move || {
        let use_login_state = login.state.read();
        if let Some(login_use_case_result) = use_login_state.data() {
            if login_use_case_result.is_login_as_patient() {
                nav.push(Route::PatientDashboard {});
            } else if login_use_case_result.is_login_as_specialist() {
                nav.push(Route::SpecialistPatients {});
            }
        }
    });

    let use_login_state = login.state.read();
    let login_result = if let Some(error) = use_login_state.error() {
        LoginResult::Error(error.to_string())
    } else if use_login_state.is_loading() {
        LoginResult::Pending
    } else {
        LoginResult::None
    };

    rsx! {
        Login {
            background_image: asset!("/assets/login.webp"),
            onsubmit: move |(email, password): (String, String)| {
                login.action.call(Credentials::from(&email, &password));
            },
            login_result,
        }
    }
}
