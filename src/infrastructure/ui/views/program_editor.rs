use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::Route;

use crate::infrastructure::app_context::AppContext;
use std::collections::HashMap;

#[component]
pub fn ProgramEditor(id: String) -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let program_id = id.clone();

    let program_id_sched = program_id.clone();
    let backend_schedule = backend.clone();
    let schedule_data = use_resource(move || {
        let pid = program_id_sched.clone();
        let backend = backend_schedule.clone();
        let session = session_signal.read().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let schedule = backend
                .list_program_schedule(sess.access_token(), &pid)
                .await?;
            let ids: Vec<String> = schedule
                .iter()
                .filter_map(|s| s.workout_id.clone())
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect();
            let workouts = backend
                .get_workouts_by_ids(sess.access_token(), &ids)
                .await
                .unwrap_or_default();
            Ok::<_, String>((schedule, workouts))
        }
    });

    let backend_library = backend.clone();
    let library_workouts = use_resource(move || {
        let backend = backend_library.clone();
        let session = session_signal.read().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let specialist_id = sess.user_id().to_string();
            backend
                .list_workout_library(sess.access_token(), &specialist_id, None)
                .await
        }
    });

    let mut schedule_block_rest = use_signal(|| true);
    let mut schedule_workout_id = use_signal(|| Option::<String>::None);
    let mut schedule_days = use_signal(|| 1i32);
    let mut schedule_add_loading = use_signal(|| false);
    let mut schedule_error = use_signal(|| Option::<String>::None);

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div {
                { t!("must_login_message") }
                " "
                Link { to: Route::LoginView {}, { t!("go_to_login") } }
            }
        };
    }

    let (schedule_items, schedule_workouts) = schedule_data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or((vec![], vec![]));
    let workout_names: HashMap<String, String> = schedule_workouts
        .iter()
        .map(|w| (w.id.clone(), w.name.clone()))
        .collect();
    let library_list = library_workouts
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let schedule_section = {
        let list_entries: Vec<Element> = schedule_items
            .iter()
            .map(|item| {
                let label = item
                    .workout_id
                    .as_ref()
                    .and_then(|id| workout_names.get(id).cloned())
                    .unwrap_or_else(|| "Descanso".to_string());
                let days = item.days_count;
                let item_id = item.id.clone();
                let backend_row = backend.clone();
                let session_signal_row = session_signal.clone();
                let mut sched_refresh = schedule_data;
                rsx! {
                    li { key: "{item_id}", class: "flex items-center gap-2 py-1 border-b border-border",
                        span { class: "font-medium", "{label}" }
                        span { class: "text-sm text-text-muted", "{days} día(s)" }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border bg-error text-white ml-auto",
                            onclick: move |_| {
                                let backend = backend_row.clone();
                                let sess = session_signal_row.read().clone();
                                let Some(s) = sess else { return };
                                let id = item_id.clone();
                                spawn(async move {
                                    let _ = backend.delete_program_schedule_item(s.access_token(), &id).await;
                                    sched_refresh.restart();
                                });
                            },
                            "Eliminar"
                        }
                    }
                }
                .into()
            })
            .collect();
        let is_rest = schedule_block_rest();
        let days_val = schedule_days().max(1);
        rsx! {
            ul { class: "list-none p-0 m-0",
                {list_entries.into_iter()}
            }
            div { class: "flex flex-col gap-4 max-w-md mt-2",
                label { style: "display: flex; align-items: center; gap: 0.5rem;",
                    input {
                        r#type: "radio",
                        name: "schedule_type",
                        checked: is_rest,
                        onchange: move |_| { schedule_block_rest.set(true); schedule_workout_id.set(None); }
                    }
                    "Días de descanso"
                }
                label { style: "display: flex; align-items: center; gap: 0.5rem;",
                    input {
                        r#type: "radio",
                        name: "schedule_type",
                        checked: !is_rest,
                        onchange: move |_| schedule_block_rest.set(false)
                    }
                    "Días de entrenamiento"
                }
                if !is_rest {
                    select {
                        onchange: move |ev| {
                            let v = ev.value();
                            schedule_workout_id.set(if v.is_empty() { None } else { Some(v) });
                        },
                        option { value: "", "Seleccionar entrenamiento" }
                        for workout in library_list.iter() {
                            option { value: "{workout.id}", "{workout.name}" }
                        }
                    }
                }
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    label { "Días: " }
                    input {
                        r#type: "number",
                        min: "1",
                        value: "{days_val}",
                        oninput: move |ev| schedule_days.set(ev.value().parse().unwrap_or(1).max(1)),
                    }
                }
                button {
                    class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                    disabled: schedule_add_loading() || (!is_rest && schedule_workout_id().is_none()),
                    onclick: move |_| {
                        let backend = backend.clone();
                        let sess = session_signal.read().clone();
                        let Some(s) = sess else { return };
                        let token = s.access_token().to_string();
                        let pid = program_id.clone();
                        let rest = schedule_block_rest();
                        let wid = schedule_workout_id();
                        let days = schedule_days().max(1);
                        let order = schedule_items.len() as i32;
                        schedule_add_loading.set(true);
                        schedule_error.set(None);
                        let mut sched_refresh = schedule_data;
                        spawn(async move {
                            let w = if rest { None } else { wid.as_deref() };
                            match backend.create_program_schedule_item(&token, &pid, order, w, days).await {
                                Ok(_) => {
                                    schedule_block_rest.set(true);
                                    schedule_workout_id.set(None);
                                    schedule_days.set(1);
                                    sched_refresh.restart();
                                }
                                Err(e) => schedule_error.set(Some(e)),
                            }
                            schedule_add_loading.set(false);
                        });
                    },
                    "Añadir bloque"
                }
            }
            if let Some(ref e) = *schedule_error.read() {
                p { class: "text-error text-sm mt-2", "{e}" }
            }
        }
    };

    rsx! {
        div { class: "view container mx-auto program-editor",
            div {
                class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                {
                    // Navbar desplegable: actúa como título de la página.
                    let mut nav_open = use_signal(|| false);
                    rsx! {
                        nav { class: "relative mb-6",
                            button {
                                class: "min-h-11 px-0 bg-transparent text-2xl font-semibold inline-flex items-center gap-2 text-text",
                                onclick: move |_| nav_open.set(!nav_open()),
                                span { "Editor de programa" }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPatients {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100", "Pacientes" }
                                    Link { to: Route::WorkoutLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100", "Biblioteca de entrenamientos" }
                                }
                            }
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 mb-6 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", "Programación (días de entrenamiento y descanso)" }
                    p { class: "text-sm text-text-muted mb-4", "Añade bloques de entrenamiento (desde tu biblioteca) o de descanso. Los entrenamientos se gestionan en la Biblioteca de entrenamientos." }
                    {schedule_section}
                }
            }
        }
    }
}
