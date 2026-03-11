use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::infrastructure::ui::components::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label,
};

#[derive(Debug, Clone, PartialEq)]
pub enum LoginResult {
    None,
    Pending,
    Error(String),
}

#[component]
pub fn Login(
    background_image: ReadSignal<Asset>,
    onsubmit: EventHandler<(String, String)>,
    login_result: LoginResult,
) -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut entered = use_signal(|| false);

    use_effect(move || {
        entered.set(true);
    });

    rsx! {
        div {
            style: "
                width: 100%;
                height: 100%;
                min-height: 100vh;
                display: flex;
                align-items: center;
                justify-content: center;
                padding: 1rem;
                background-size: cover;
                background-position: center;
                background-repeat: no-repeat;
            ",
            background_image: "url('{background_image}')",
            div {
                style: if entered() {
                    "
                        opacity: 0.9;
                        transform: scale(1);
                        transition: opacity 220ms ease-out, transform 220ms ease-out;
                        padding-top: 2rem;
                        max-width: 22rem;
                        width: 100%;
                        margin-left: auto;
                        margin-right: auto;
                    "
                } else {
                    "
                        opacity: 0;
                        transform: scale(0.9);
                        transition: opacity 220ms ease-out, transform 220ms ease-out;
                        padding-top: 2rem;
                        max-width: 22rem;
                        width: 100%;
                        margin-left: auto;
                        margin-right: auto;
                    "
                },
                Card {
                    CardHeader {
                        CardTitle {
                            { t!("login_title") }
                        }
                        CardDescription {
                            { t!("login_description") }
                        }
                        match &login_result {
                            LoginResult::Error(err) => {
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
                                onsubmit.call((email(), password()));
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
                        Button {
                            class: if matches!(login_result, LoginResult::Pending) { "opacity-50 !cursor-not-allowed" } else { "" },
                            r#type: "submit",
                            form: "login-form",
                            disabled: matches!(login_result, LoginResult::Pending),
                            { t!("login_button_label") },
                        }
                    }
                }
            }
        }
    }
}
