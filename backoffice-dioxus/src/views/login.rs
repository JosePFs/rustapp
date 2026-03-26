use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::components::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label,
};
use crate::hooks::{login::use_login, AsyncState};
use crate::Route;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    let mut login = use_login();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    if login
        .state
        .read()
        .data()
        .map(|data| data.is_login_as_specialist())
        .unwrap_or(false)
    {
        nav.push(Route::SpecialistPatients {});
        return rsx! {};
    }

    let background_image = asset!("/assets/login.webp");

    rsx! {
            div {
                style: "
                width: 100%;
                height: 100%;
                min-height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                background-size: cover;
                background-position: center;
                background-repeat: no-repeat;
                background-image: url('{background_image}');
                overflow: hidden;
                padding: 0;
                margin: 0;
            ",
            div {
                style: "
                    max-width: 22rem;
                    width: 100%;
                    margin-left: auto;
                    margin-right: auto;
                    opacity: 0;
                    animation: login-card-enter 380ms ease-out forwards;
                ",
                Card {
                    CardHeader {
                        CardTitle {
                            { t!("login_title") }
                        }
                        CardDescription {
                            { t!("login_description") }
                        }
                        match &*login.state.read() {
                            AsyncState::Error(err) => {
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
                            class: "flex flex-col gap-4",
                            onsubmit: move |ev: FormEvent| {
                                ev.prevent_default();
                                login.action.call((email(), password()));
                            },
                            div {
                                class: "grid gap-2",
                                Label {
                                    html_for: "email",
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
                            }
                            div {
                                class: "grid gap-2",
                                Label {
                                    html_for: "password",
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
                    }
                    CardFooter {
                        Button { class: if login.state.read().is_loading() { "opacity-50 !cursor-not-allowed" } else { "" },
                            style: "color: var(--primary-color);background-color: var(--secondary-color-2);cursor: pointer;border: none;border-radius: 0.5rem;padding: 8px 18px;font-size: 1rem;transition: background-color 0.2s ease, color 0.2s ease;",
                            r#type: "submit",
                            form: "login-form",
                            disabled: login.state.read().is_loading(),
                            { t!("login_button_label") },
                        }
                    }
                }
            }
        }
    }
}
