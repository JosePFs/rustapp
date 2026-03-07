use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::domain::{credentials::Credentials, role::Role};
use crate::infrastructure::ui::hooks::login::use_login;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let (mut login_action, profile_loader) = use_login();

    use_memo(move || {
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

    rsx! {
        div {
            class: "view login w-full h-full min-h-screen flex items-center justify-center p-4 bg-cover bg-center bg-no-repeat",
            background_image: "url('{asset!(\"/assets/login.webp\")}')",
            div {
                class: "content opacity-90 pt-8 max-w-[22rem] w-full mx-auto",
                h1 { class: "text-xl font-semibold text-center mb-4", { t!("login_title") } },
                match login_action.value() {
                    Some(Err(err)) => {
                        rsx! {
                            p { class: "text-error text-sm mb-4 p-2 px-4 rounded-md bg-red-50 border border-red-200", "{err}" }
                        }
                    },
                    _ => { rsx! { } }
                },
                form {
                    class: "flex flex-col gap-4 bg-surface p-6 rounded-lg border border-border shadow-sm",
                    onsubmit: move |ev| {
                        ev.prevent_default();
                        login_action.call(Credentials::from(&email(), &password()));
                    },
                    label {
                        class: "flex flex-col gap-1 text-sm font-medium",
                        { t!("email_label") },
                        input {
                            class: "w-full min-h-11 px-4 text-base border border-border rounded-md bg-surface focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20",
                            r#type: "email",
                            placeholder: "email@ejemplo.com",
                            value: "{email}",
                            required: true,
                            oninput: move |ev| email.set(ev.value().clone()),
                        }
                    }
                    label {
                        class: "flex flex-col gap-1 text-sm font-medium",
                        { t!("password_label") },
                        input {
                            class: "w-full min-h-11 px-4 text-base border border-border rounded-md bg-surface focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: "{password}",
                            required: true,
                            oninput: move |ev| password.set(ev.value().clone()),
                        }
                    }
                    button {
                        class: "mt-6 w-full min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                        r#type: "submit",
                        disabled: login_action.pending(),
                        { t!("login_button_label") },
                    }
                }
            }
        }
    }
}
