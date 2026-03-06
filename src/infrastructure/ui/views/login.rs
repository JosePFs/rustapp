use dioxus::prelude::*;
use dioxus::CapturedError;
use dioxus_router::use_navigator;

use crate::domain::credentials::Credentials;
use crate::domain::role::Role;
use crate::infrastructure::ui::hooks::app_context::use_app_context;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let app_context = use_app_context();
    let mut app_session = app_context.session();
    let backend = app_context.backend();

    let backend_for_action = backend.clone();
    let mut login_action = use_action(move |credentials: Credentials| {
        let backend = backend_for_action.clone();

        async move {
            backend
                .sign_in(&credentials)
                .await
                .map_err(Into::<CapturedError>::into)
        }
    });

    let backend_for_profile_loader = backend.clone();
    use_effect(move || {
        let backend = backend_for_profile_loader.clone();
        let login_action = login_action.value();

        spawn(async move {
            let Some(Ok(session)) = login_action else {
                return;
            };

            let profiles = backend
                .get_profiles_by_ids(&[session().user_id().to_string()], session().access_token())
                .await
                .ok();

            let is_specialist = profiles
                .and_then(|p| p.into_iter().next())
                .map(|p| p.role() == &Role::Specialist)
                .unwrap_or(false);

            app_session.set(Some(session()));

            if is_specialist {
                nav.push(Route::SpecialistDashboard {});
            } else {
                nav.push(Route::PatientDashboard {});
            }
        });
    });

    let bg_image_url = asset!("/assets/login.webp");
    rsx! {
        div {
            class: "view login min-h-screen flex items-center justify-center p-4 bg-cover bg-center bg-no-repeat",
            background_image: "url('{bg_image_url}')",
            div {
                class: "content opacity-90 pt-8 max-w-[22rem] w-full mx-auto",
                h1 { class: "text-xl font-semibold text-center mb-4", "Iniciar sesión" },
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
                        "Email"
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
                        "Contraseña"
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
                        "Entrar"
                    }
                }
            }
        }
    }
}
