use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::domain::{credentials::Credentials, role::Role};
use crate::infrastructure::ui::components::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label,
};
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
                Card {
                    CardHeader {
                        CardTitle {
                            { t!("login_title") }
                        }
                        CardDescription {
                            { t!("login_description") }
                        }
                        match login_action.value() {
                            Some(Err(err)) => {
                                rsx! {
                                    p { class: "text-error text-sm p-2 rounded-md bg-red-50 border border-red-200", "{err}" }
                                }
                            },
                            _ => { rsx! { } }
                        }
                    }
                    CardContent {
                        form {
                            id: "login-form",
                            class: "flex flex-col gap-4 bg-surface p-6 rounded-lg border border-border shadow-sm",
                            onsubmit: move |ev| {
                                ev.prevent_default();
                                login_action.call(Credentials::from(&email(), &password()));
                            },
                            Label {
                                html_for: "email",
                                class: "flex flex-col text-sm font-medium",
                                { t!("email_label") },
                            }
                            Input {
                                id: "email",
                                autocomplete: "email",
                                r#type: "email",
                                placeholder: "email@ejemplo.com",
                                value: "{email}",
                                required: true,
                                oninput: move |ev: FormEvent| email.set(ev.value().clone()),
                            }
                            Label {
                                html_for: "password",
                                class: "flex flex-col text-sm font-medium",
                                { t!("password_label") },
                            }
                            Input {
                                id: "password",
                                autocomplete: "current-password",
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{password}",
                                required: true,
                                oninput: move |ev: FormEvent| password.set(ev.value().clone()),
                            }
                        }
                    }
                    CardFooter {
                        Button {
                            class: if login_action.pending() { "opacity-50 !cursor-not-allowed" } else { "" },
                            r#type: "submit",
                            form: "login-form",
                            disabled: login_action.pending(),
                            { t!("login_button_label") },
                        }
                    }
                }
            }
        }
    }
}
