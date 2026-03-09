//! Specialist's workout library: create, edit, delete, filter. Workouts are reusable in programs.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::infrastructure::app_context::AppContext;
use crate::Route;

#[component]
pub fn WorkoutLibrary() -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let mut filter = use_signal(|| String::new());

    let backend_for_resource = backend.clone();
    let mut workouts = use_resource(move || {
        let backend = backend_for_resource.clone();
        let session = session_signal.read().clone();
        let filter_val = filter().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let specialist_id = sess.user_id().to_string();
            backend
                .list_workout_library(
                    sess.access_token(),
                    &specialist_id,
                    if filter_val.is_empty() {
                        None
                    } else {
                        Some(filter_val.as_str())
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
            div { "Debes iniciar sesión. " Link { to: Route::LoginView {}, "Ir a login" } }
        };
    }

    let list = workouts
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let list_len = list.len();
    let empty_ok = workouts
        .read()
        .as_ref()
        .as_ref()
        .map(|r| r.is_ok())
        .unwrap_or(false);
    let backend_for_rows = backend.clone();
    let rows: Vec<Element> = list
        .into_iter()
        .map(|wo| {
            let wid = wo.id.clone();
            let wid_edit = wo.id.clone();
            let wid_del = wo.id.clone();
            let name = wo.name.clone();
            let desc = wo.description.clone().unwrap_or_default();
            let backend_row = backend_for_rows.clone();
            rsx! {
                li { key: "{wid}", class: "p-4 bg-surface border border-border rounded-md mb-2",
                    if editing_id().as_ref() == Some(&wid_edit) {
                        div { class: "flex flex-wrap gap-2 items-center mt-2",
                            input {
                                class: "flex-1 min-w-32 min-h-9 px-3 border border-border rounded-md text-sm",
                                placeholder: "Nombre",
                                value: "{edit_name()}",
                                oninput: move |ev| edit_name.set(ev.value().clone()),
                            }
                            input {
                                class: "flex-1 min-w-32 min-h-9 px-3 border border-border rounded-md text-sm",
                                placeholder: "Descripción (opcional)",
                                value: "{edit_desc()}",
                                oninput: move |ev| edit_desc.set(ev.value().clone()),
                            }
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md bg-primary text-white",
                                onclick: move |_| {
                                    let backend = backend_row.clone();
                                    let sess = session_signal.read().clone();
                                    let Some(s) = sess else { return };
                                    let token = s.access_token().to_string();
                                    let id = wid_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    editing_id.set(None);
                                    let mut refresh = workouts;
                                    spawn(async move {
                                        let _ = backend.update_workout(&token, &id, Some(&n), Some(if d.is_empty() { None } else { Some(d.as_str()) }), None).await;
                                        refresh.restart();
                                    });
                                },
                                "Guardar"
                            }
                            button { class: "min-h-9 px-2 text-sm rounded-md border border-border", onclick: move |_| editing_id.set(None), "Cancelar" }
                        }
                    } else {
                        span { class: "block",
                            strong { "{name}" }
                            if !desc.is_empty() { span { " — {desc}" } }
                        }
                        Link {
                            to: Route::WorkoutEditor { id: wid.clone() },
                            class: "inline-block min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2 text-primary no-underline hover:bg-gray-50",
                            "Ejercicios"
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2",
                            onclick: move |_| {
                                edit_name.set(name.clone());
                                edit_desc.set(desc.clone());
                                editing_id.set(Some(wid_edit.clone()));
                            },
                            "Editar"
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md bg-error text-white mt-2",
                            onclick: move |_| {
                                let backend = backend_row.clone();
                                let sess = session_signal.read().clone();
                                let Some(s) = sess else { return };
                                let id = wid_del.clone();
                                let mut refresh = workouts;
                                spawn(async move {
                                    let _ = backend.delete_workout(s.access_token(), &id).await;
                                    refresh.restart();
                                });
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
        div { class: "view container mx-auto workout-library flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                {
                    // Navbar desplegable: actúa como título de la página.
                    let mut nav_open = use_signal(|| false);
                    rsx! {
                        nav { class: "relative mb-6",
                            button {
                                class: "min-h-11 px-0 bg-transparent text-2xl font-semibold inline-flex items-center gap-2 text-text",
                                onclick: move |_| nav_open.set(!nav_open()),
                                span { "Biblioteca de entrenamientos" }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPatients {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Pacientes" }
                                    Link { to: Route::ExerciseLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de ejercicios" }
                                }
                            }
                        }
                    }
                }
                p { class: "text-sm text-text-muted mb-4", "Crea y edita entrenamientos aquí. Luego añádelos a programas desde el editor del programa (programación)." }
                input {
                    class: "w-full min-h-11 px-4 border border-border rounded-md mb-4 focus:outline-none focus:border-primary",
                    placeholder: "Filtrar por nombre...",
                    value: "{filter()}",
                    oninput: move |ev| { filter.set(ev.value().clone()); workouts.restart(); },
                }
                section { class: "bg-surface rounded-lg p-4 mb-6 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", "Nuevo entrenamiento" }
                    div { class: "flex flex-col gap-4 max-w-md",
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "Nombre",
                            value: "{new_name()}",
                            oninput: move |ev| new_name.set(ev.value().clone()),
                        }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "Descripción (opcional)",
                            value: "{new_desc()}",
                            oninput: move |ev| new_desc.set(ev.value().clone()),
                        }
                        button {
                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                            disabled: create_loading() || new_name().trim().is_empty(),
                            onclick: move |_| {
                                let name = new_name().trim().to_string();
                                if name.is_empty() { return; }
                                let backend = backend.clone();
                                let session = session_signal.read().clone();
                                let Some(s) = session else { return };
                                let token = s.access_token().to_string();
                                let specialist_id = s.user_id().to_string();
                                let desc = new_desc().clone();
                                create_loading.set(true);
                                create_error.set(None);
                                let mut refresh = workouts;
                                spawn(async move {
                                    match backend.create_workout(
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
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", "Entrenamientos ({list_len})" }
                    ul { class: "list-none p-0 m-0",
                        {rows.into_iter()}
                    }
                    if list_len == 0 && empty_ok {
                        p { class: "text-text-muted italic py-4", "Aún no hay entrenamientos. Crea uno arriba." }
                    }
                }
            }
        }
    }
}
