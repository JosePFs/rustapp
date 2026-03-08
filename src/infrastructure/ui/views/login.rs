use dioxus::prelude::*;
use dioxus_router::use_navigator;

use crate::domain::{credentials::Credentials, role::Role};
use crate::infrastructure::ui::components::{Login, LoginResult};
use crate::infrastructure::ui::hooks::login::use_login;
use crate::Route;

#[component]
pub fn LoginView() -> Element {
    let nav = use_navigator();
    let (mut login_action, profile_loader) = use_login();

    use_effect(move || {
        let profiles = profile_loader.read();

        if let Some(Some(profiles)) = profiles.as_ref() {
            let is_specialist = profiles
                .into_iter()
                .next()
                .map(|p| p.role() == &Role::Specialist)
                .unwrap_or(false);
            if is_specialist {
                nav.push(Route::SpecialistDashboard {});
            } else {
                nav.push(Route::PatientDashboard {});
            }
        }
    });

    let login_result = if login_action.pending() {
        LoginResult::Pending
    } else {
        login_action
            .value()
            .map(|result| match result {
                Ok(_) => LoginResult::Success,
                Err(err) => LoginResult::Error(err.to_string()),
            })
            .unwrap_or(LoginResult::None)
    };

    rsx! {
        Login {
            background_image: asset!("/assets/login.webp"),
            onsubmit: move |(email, password): (String, String)| {
                login_action.call(Credentials::from(&email, &password));
            },
            login_result,
        }
    }
}
