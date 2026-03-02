//! Specialist's exercise library: create, edit, delete, filter. Exercises are reusable in workouts.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;
use crate::services::data::{
    create_exercise, list_exercise_library, restore_exercise, soft_delete_exercise, update_exercise,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[component]
pub fn ExerciseLibrary() -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let mut filter = use_signal(|| String::new());

    let mut exercises = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        let filter_val = filter().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let specialist_id = sess.user.id.clone();
            list_exercise_library(
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
    let mut new_video_url = use_signal(|| String::new());
    let mut create_loading = use_signal(|| false);
    let mut create_error = use_signal(|| Option::<String>::None);
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_desc = use_signal(|| String::new());
    let mut edit_video_url = use_signal(|| String::new());

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let list = exercises
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let list_len = list.len();
    let empty_ok = exercises.read().as_ref().as_ref().map(|r| r.is_ok()).unwrap_or(false);
    let rows: Vec<Element> = list
        .into_iter()
        .map(|ex| {
            let ex_id = ex.id.clone();
            let ex_id_edit = ex_id.clone();
            let ex_id_del = ex_id.clone();
            let ex_id_restore = ex_id.clone();
            let name = ex.name.clone();
            let desc = ex.description.clone().unwrap_or_default();
            let video = ex.video_url.clone().unwrap_or_default();
            let is_deleted = ex.deleted_at.is_some();
            rsx! {
                li { key: "{ex_id}",
                    class: if is_deleted { "deleted" } else { "" },
                    if editing_id().as_ref() == Some(&ex_id_edit) {
                        div { class: "edit-form",
                            input {
                                value: "{edit_name()}",
                                oninput: move |ev| edit_name.set(ev.value().clone()),
                            }
                            input {
                                value: "{edit_desc()}",
                                oninput: move |ev| edit_desc.set(ev.value().clone()),
                            }
                            input {
                                value: "{edit_video_url()}",
                                oninput: move |ev| edit_video_url.set(ev.value().clone()),
                            }
                            button {
                                onclick: move |_| {
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let sess = session_signal.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let eid = ex_id_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    let v = edit_video_url().clone();
                                    editing_id.set(None);
                                    let mut refresh = exercises;
                                    spawn(async move {
                                        let _ = update_exercise(&cfg, &token, &eid, Some(&n), Some(if d.is_empty() { "" } else { &d }), None, Some(if v.is_empty() { None } else { Some(v.as_str()) })).await;
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
                            if is_deleted {
                                span { class: "badge", " (eliminado)" }
                            }
                        }
                        if !is_deleted {
                            button {
                                class: "btn-small",
                                onclick: move |_| {
                                    edit_name.set(name.clone());
                                    edit_desc.set(desc.clone());
                                    edit_video_url.set(video.clone());
                                    editing_id.set(Some(ex_id_edit.clone()));
                                },
                                "Editar"
                            }
                            button {
                                class: "btn-small danger",
                                onclick: move |_| {
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let sess = session_signal.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let eid = ex_id_del.clone();
                                    let mut refresh = exercises;
                                    spawn(async move {
                                        let _ = soft_delete_exercise(&cfg, &token, &eid).await;
                                        refresh.restart();
                                    });
                                },
                                "Eliminar"
                            }
                        } else {
                            button {
                                class: "btn-small",
                                onclick: move |_| {
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let sess = session_signal.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let eid = ex_id_restore.clone();
                                    let mut refresh = exercises;
                                    spawn(async move {
                                        let _ = restore_exercise(&cfg, &token, &eid).await;
                                        refresh.restart();
                                    });
                                },
                                "Restaurar"
                            }
                        }
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "exercise-library",
            h1 { "Biblioteca de ejercicios" }
            nav { class: "nav",
                Link { to: Route::SpecialistDashboard {}, "← Panel del especialista" }
            }
            p { class: "hint", "Crea y edita ejercicios aquí. Luego añádelos a entrenamientos desde el editor del programa." }
            input {
                class: "filter-input",
                placeholder: "Filtrar por nombre...",
                value: "{filter()}",
                oninput: move |ev| { filter.set(ev.value().clone()); exercises.restart(); },
            }
            section { class: "create-form",
                h2 { "Nuevo ejercicio" }
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
                    input {
                        placeholder: "URL vídeo YouTube (opcional)",
                        value: "{new_video_url()}",
                        oninput: move |ev| new_video_url.set(ev.value().clone()),
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
                            let video = new_video_url().clone();
                            create_loading.set(true);
                            create_error.set(None);
                            let mut refresh = exercises;
                            spawn(async move {
                                match create_exercise(
                                    &cfg,
                                    &token,
                                    &specialist_id,
                                    &name,
                                    if desc.is_empty() { None } else { Some(&desc) },
                                    0,
                                    if video.is_empty() { None } else { Some(&video) },
                                ).await {
                                    Ok(_) => {
                                        new_name.set(String::new());
                                        new_desc.set(String::new());
                                        new_video_url.set(String::new());
                                        refresh.restart();
                                    }
                                    Err(e) => create_error.set(Some(e)),
                                }
                                create_loading.set(false);
                            });
                        },
                        "Crear ejercicio"
                    }
                    if let Some(ref e) = *create_error.read() {
                        p { class: "error", "{e}" }
                    }
                }
            }
            section {
                h2 { "Ejercicios ({list_len})" }
                ul { class: "exercise-list",
                    {rows.into_iter()}
                }
                if list_len == 0 && empty_ok {
                    p { "Aún no hay ejercicios. Crea uno arriba." }
                }
            }
        }
    }
}
