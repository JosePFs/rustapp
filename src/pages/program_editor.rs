//! Program editor: only programación (schedule). Add blocks from workout library or rest days.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;

use crate::services::data::{
    create_program_schedule_item, delete_program_schedule_item, get_workouts_by_ids,
    list_program_schedule, list_workout_library,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};
use std::collections::HashMap;

#[component]
pub fn ProgramEditor(id: String) -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let program_id = id.clone();

    let program_id_sched = program_id.clone();
    let schedule_data = use_resource(move || {
        let pid = program_id_sched.clone();
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let schedule = list_program_schedule(&cfg, &sess.access_token, &pid).await?;
            let ids: Vec<String> = schedule
                .iter()
                .filter_map(|s| s.workout_id.clone())
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect();
            let workouts = get_workouts_by_ids(&cfg, &sess.access_token, &ids).await.unwrap_or_default();
            Ok::<_, String>((schedule, workouts))
        }
    });

    let library_workouts = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let specialist_id = sess.user.id.clone();
            list_workout_library(&cfg, &sess.access_token, &specialist_id, None).await
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
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
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
                let config_signal = config_signal;
                let session_signal = session_signal;
                let mut sched_refresh = schedule_data;
                rsx! {
                    li { key: "{item_id}", style: "display: flex; align-items: center; gap: 0.5rem; margin: 0.25rem 0;",
                        span { class: "schedule-label", "{label}" }
                        span { class: "schedule-days", "{days} día(s)" }
                        button {
                            class: "btn-small",
                            onclick: move |_| {
                                let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                let sess = session_signal.read().clone();
                                if let (Some(cfg), Some(s)) = (config, sess) {
                                    let id = item_id.clone();
                                    spawn(async move {
                                        let _ = delete_program_schedule_item(&cfg, &s.access_token, &id).await;
                                        sched_refresh.restart();
                                    });
                                }
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
            ul { class: "schedule-list", style: "list-style: none; padding: 0;",
                {list_entries.into_iter()}
            }
            div { class: "form", style: "display: flex; flex-direction: column; gap: 0.5rem; max-width: 400px; margin-top: 0.5rem;",
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
                    disabled: schedule_add_loading() || (!is_rest && schedule_workout_id().is_none()),
                    onclick: move |_| {
                        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                        let sess = session_signal.read().clone();
                        let (cfg, token) = match (config, sess) {
                            (Some(c), Some(s)) => (c, s.access_token),
                            _ => return,
                        };
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
                            match create_program_schedule_item(&cfg, &token, &pid, order, w, days).await {
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
                p { class: "error", "{e}" }
            }
        }
    };

    rsx! {
        div { class: "program-editor",
            h1 { "Editor de programa" }
            nav {
                Link { to: Route::SpecialistDashboard {}, "Volver al panel" }
                Link { to: Route::WorkoutLibrary {}, "Biblioteca de entrenamientos" }
            }
            section { class: "program-schedule",
                h2 { "Programación (días de entrenamiento y descanso)" }
                p { class: "hint", "Añade bloques de entrenamiento (desde tu biblioteca) o de descanso. Los entrenamientos se gestionan en la Biblioteca de entrenamientos." }
                {schedule_section}
            }
        }
    }
}
