//! Edit a workout from the library: view exercises, add from library, remove from workout.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;
use crate::services::data::{
    add_exercise_to_workout, get_workouts_by_ids, list_exercise_library, list_exercises_for_workout,
    remove_exercise_from_workout, Exercise,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[component]
pub fn WorkoutEditor(id: String) -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let id_workout = id.clone();
    let id_exercises = id.clone();

    let workout = use_resource(move || {
        let wid = id_workout.clone();
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let rows = get_workouts_by_ids(&cfg, &sess.access_token, &[wid]).await?;
            Ok(rows.into_iter().next())
        }
    });

    let exercises = use_resource(move || {
        let wid = id_exercises.clone();
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            list_exercises_for_workout(&cfg, &sess.access_token, &wid).await
        }
    });

    let library = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let specialist_id = sess.user.id.clone();
            list_exercise_library(&cfg, &sess.access_token, &specialist_id, None).await
        }
    });

    let mut add_exercise_id = use_signal(|| Option::<String>::None);
    let mut add_loading = use_signal(|| false);

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let workout_opt = workout
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().and_then(|o| o.clone()));
    let exs = exercises
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();
    let library_list = library
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();
    let ex_ids_in_workout: std::collections::HashSet<String> = exs.iter().map(|e| e.id.clone()).collect();
    let available_to_add: Vec<&Exercise> = library_list
        .iter()
        .filter(|e| !ex_ids_in_workout.contains(&e.id) && e.deleted_at.is_none())
        .collect();

    let exercise_rows: Vec<Element> = exs
        .iter()
        .map(|e| {
            let eid = e.id.clone();
            let ename = e.name.clone();
            let deleted = e.deleted_at.is_some();
            let wid = id.clone();
            let mut ex_refresh = exercises;
            rsx! {
                li { key: "{eid}",
                    span { "{ename}" }
                    if deleted {
                        span { class: "badge", " (eliminado en biblioteca)" }
                    }
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                            let sess = session_signal.read().clone();
                            if let (Some(cfg), Some(s)) = (config, sess) {
                                let exercise_id = eid.clone();
                                let workout_id_for_remove = wid.clone();
                                spawn(async move {
                                    let _ = remove_exercise_from_workout(&cfg, &s.access_token, &workout_id_for_remove, &exercise_id).await;
                                    ex_refresh.restart();
                                });
                            }
                        },
                        "Quitar"
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "workout-editor",
            h1 { "Entrenamiento" }
            nav { class: "nav",
                Link { to: Route::WorkoutLibrary {}, "← Biblioteca de entrenamientos" }
            }
            if let Some(ref w) = workout_opt {
                h2 { "{w.name}" }
                if let Some(ref d) = w.description {
                    if !d.is_empty() {
                        p { "{d}" }
                    }
                }
            } else if workout.read().as_ref().as_ref().map(|r| r.is_ok()).unwrap_or(false) {
                p { "Entrenamiento no encontrado." }
            } else {
                p { "Cargando..." }
            }
            if workout_opt.is_some() {
                section {
                    h3 { "Ejercicios en este entrenamiento" }
                    ul { class: "exercise-list",
                        {exercise_rows.into_iter()}
                    }
                    if exs.is_empty() {
                        p { class: "muted", "Aún no hay ejercicios. Añade desde la biblioteca abajo." }
                    }
                }
                section {
                    h3 { "Añadir desde biblioteca de ejercicios" }
                    if available_to_add.is_empty() {
                        p { class: "muted", "Todos los ejercicios ya están en este entrenamiento o no hay ejercicios en la biblioteca." }
                    } else {
                        select {
                            onchange: move |ev| {
                                let v = ev.value();
                                add_exercise_id.set(if v.is_empty() { None } else { Some(v) });
                            },
                            option { value: "", "Seleccionar ejercicio" }
                            for exercise in available_to_add.iter() {
                                option { value: "{exercise.id}", "{exercise.name}" }
                            }
                        }
                        button {
                            disabled: add_loading() || add_exercise_id().is_none(),
                            onclick: move |_| {
                                let eid = match add_exercise_id() {
                                    Some(eid) => eid,
                                    None => return,
                                };
                                let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                let sess = session_signal.read().clone();
                                if let (Some(cfg), Some(s)) = (config, sess) {
                                    let wid = id.clone();
                                    add_loading.set(true);
                                    let mut ex_refresh = exercises;
                                    spawn(async move {
                                        let order = 0;
                                        let _ = add_exercise_to_workout(&cfg, &s.access_token, &wid, &eid, order).await;
                                        ex_refresh.restart();
                                        add_loading.set(false);
                                    });
                                }
                            },
                            "Añadir al entrenamiento"
                        }
                    }
                }
            }
        }
    }
}
