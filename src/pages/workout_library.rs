//! Specialist's workout library: create, edit, delete, filter. Workouts are reusable in programs.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;
use crate::services::data::{
    create_workout, delete_workout, list_workout_library, update_workout,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[component]
pub fn WorkoutLibrary() -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let mut filter = use_signal(|| String::new());

    let mut workouts = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        let filter_val = filter().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let specialist_id = sess.user.id.clone();
            list_workout_library(
                &cfg,
                &sess.access_token,
                &specialist_id,
                if filter_val.is_empty() {
                    None
                } else {
                    Some(&filter_val)
                },
            )
            .await
        }
    });

    let mut new_name = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut create_loading = use_signal(|| false);
    let mut create_error = use_signal(|| Option::<String>::None);
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_desc = use_signal(|| String::new());

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let list = workouts
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let list_len = list.len();
    let empty_ok = workouts.read().as_ref().as_ref().map(|r| r.is_ok()).unwrap_or(false);
    let rows: Vec<Element> = list
        .into_iter()
        .map(|wo| {
            let wid = wo.id.clone();
            let wid_edit = wo.id.clone();
            let wid_del = wo.id.clone();
            let name = wo.name.clone();
            let desc = wo.description.clone().unwrap_or_default();
            rsx! {
                li { key: "{wid}",
                    if editing_id().as_ref() == Some(&wid_edit) {
                        div { class: "edit-form",
                            input {
                                placeholder: "Nombre",
                                value: "{edit_name()}",
                                oninput: move |ev| edit_name.set(ev.value().clone()),
                            }
                            input {
                                placeholder: "Descripción (opcional)",
                                value: "{edit_desc()}",
                                oninput: move |ev| edit_desc.set(ev.value().clone()),
                            }
                            button {
                                onclick: move |_| {
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let sess = session_signal.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let id = wid_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    editing_id.set(None);
                                    let mut refresh = workouts;
                                    spawn(async move {
                                        let _ = update_workout(&cfg, &token, &id, Some(&n), Some(if d.is_empty() { None } else { Some(d.as_str()) }), None).await;
                                        refresh.restart();
                                    });
                                },
                                "Guardar"
                            }
                            button { onclick: move |_| editing_id.set(None), "Cancelar" }
                        }
                    } else {
                        span {
                            strong { "{name}" }
                            if !desc.is_empty() { span { " — {desc}" } }
                        }
                        Link {
                            to: Route::WorkoutEditor { id: wid.clone() },
                            class: "btn-small",
                            "Ejercicios"
                        }
                        button {
                            class: "btn-small",
                            onclick: move |_| {
                                edit_name.set(name.clone());
                                edit_desc.set(desc.clone());
                                editing_id.set(Some(wid_edit.clone()));
                            },
                            "Editar"
                        }
                        button {
                            class: "btn-small danger",
                            onclick: move |_| {
                                let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                let sess = session_signal.read().clone();
                                if let (Some(cfg), Some(s)) = (config, sess) {
                                    let id = wid_del.clone();
                                    let mut refresh = workouts;
                                    spawn(async move {
                                        let _ = delete_workout(&cfg, &s.access_token, &id).await;
                                        refresh.restart();
                                    });
                                }
                            },
                            "Eliminar"
                        }
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "workout-library",
            h1 { "Biblioteca de entrenamientos" }
            nav { class: "nav",
                Link { to: Route::SpecialistDashboard {}, "← Panel del especialista" }
                Link { to: Route::ExerciseLibrary {}, "Biblioteca de ejercicios" }
            }
            p { class: "hint", "Crea y edita entrenamientos aquí. Luego añádelos a programas desde el editor del programa (programación)." }
            input {
                class: "filter-input",
                placeholder: "Filtrar por nombre...",
                value: "{filter()}",
                oninput: move |ev| { filter.set(ev.value().clone()); workouts.restart(); },
            }
            section { class: "create-form",
                h2 { "Nuevo entrenamiento" }
                div { class: "form", style: "display: flex; flex-direction: column; gap: 0.5rem; max-width: 400px;",
                    input {
                        placeholder: "Nombre",
                        value: "{new_name()}",
                        oninput: move |ev| new_name.set(ev.value().clone()),
                    }
                    input {
                        placeholder: "Descripción (opcional)",
                        value: "{new_desc()}",
                        oninput: move |ev| new_desc.set(ev.value().clone()),
                    }
                    button {
                        disabled: create_loading() || new_name().trim().is_empty(),
                        onclick: move |_| {
                            let name = new_name().trim().to_string();
                            if name.is_empty() { return; }
                            let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                            let session = session_signal.read().clone();
                            let (cfg, token, specialist_id) = match (config, session) {
                                (Some(c), Some(s)) => (c, s.access_token, s.user.id.clone()),
                                _ => return,
                            };
                            let desc = new_desc().clone();
                            create_loading.set(true);
                            create_error.set(None);
                            let mut refresh = workouts;
                            spawn(async move {
                                match create_workout(
                                    &cfg,
                                    &token,
                                    &specialist_id,
                                    &name,
                                    if desc.is_empty() { None } else { Some(&desc) },
                                ).await {
                                    Ok(_) => {
                                        new_name.set(String::new());
                                        new_desc.set(String::new());
                                        refresh.restart();
                                    }
                                    Err(e) => create_error.set(Some(e)),
                                }
                                create_loading.set(false);
                            });
                        },
                        "Crear entrenamiento"
                    }
                    if let Some(ref e) = *create_error.read() {
                        p { class: "error", "{e}" }
                    }
                }
            }
            section {
                h2 { "Entrenamientos ({list_len})" }
                ul { class: "workout-list",
                    {rows.into_iter()}
                }
                if list_len == 0 && empty_ok {
                    p { "Aún no hay entrenamientos. Crea uno arriba." }
                }
            }
        }
    }
}
