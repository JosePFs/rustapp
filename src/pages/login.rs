//! Login page — Supabase email/password auth.

use dioxus::prelude::*;
use dioxus_router::{use_navigator, Link};

use crate::services::data::get_profiles_by_ids;
use crate::services::supabase_client::{sign_in, AuthSession, SupabaseConfig};
use crate::Route;

/// Session stored after login; can be provided via context for the app.
pub type AppSession = Option<AuthSession>;

#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let mut session_signal = use_context::<Signal<Option<AuthSession>>>();
    let config = config_signal
        .read()
        .clone()
        .or_else(SupabaseConfig::from_env);

    rsx! {
        div { class: "login-page",
            h1 { "Iniciar sesión" }
            if let Some(ref err) = *error.read() {
                p { class: "error", "{err}" }
            }
            form {
                onsubmit: move |ev| {
                    ev.prevent_default();
                    let email_val = email.read().clone();
                    let password_val = password.read().clone();
                    if email_val.is_empty() || password_val.is_empty() {
                        error.set(Some("Email y contraseña requeridos".into()));
                        return;
                    }
                    let config = config.clone();
                    let nav = nav.clone();
                    loading.set(true);
                    error.set(None);
                    spawn(async move {
                        let Some(cfg) = config else {
                            error.set(Some("SUPABASE_URL y SUPABASE_ANON_KEY no configurados".into()));
                            loading.set(false);
                            return;
                        };
                        match sign_in(&cfg, &email_val, &password_val).await {
                            Ok(session) => {
                                session_signal.set(Some(session.clone()));
                                loading.set(false);
                                // Rol desde la API (profiles), no del JWT, para que coincida con la BD
                                let is_specialist = get_profiles_by_ids(
                                    &cfg,
                                    &session.access_token,
                                    &[session.user.id.clone()],
                                )
                                .await
                                .ok()
                                .and_then(|profiles| profiles.into_iter().next())
                                .map(|p| p.role == "specialist")
                                .unwrap_or(false);
                                let _ = if is_specialist {
                                    nav.push(Route::SpecialistDashboard {})
                                } else {
                                    nav.push(Route::PatientDashboard {})
                                };
                            }
                            Err(e) => {
                                error.set(Some(e));
                                loading.set(false);
                            }
                        }
                    });
                },
                label { "Email"
                    input {
                        r#type: "email",
                        placeholder: "email@ejemplo.com",
                        value: "{email()}",
                        oninput: move |ev| email.set(ev.value().clone()),
                    }
                }
                label { "Contraseña"
                    input {
                        r#type: "password",
                        placeholder: "••••••••",
                        value: "{password()}",
                        oninput: move |ev| password.set(ev.value().clone()),
                    }
                }
                button { r#type: "submit", disabled: loading(), "Entrar" }
            }
        }
    }
}
