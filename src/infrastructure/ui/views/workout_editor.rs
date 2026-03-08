//! Edit a workout from the library: view exercises, add from library, remove from workout.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::domain::entities::Exercise;
use crate::infrastructure::app_context::AppContext;
use crate::Route;

#[component]
pub fn WorkoutEditor(id: String) -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let id_workout = id.clone();
    let id_exercises = id.clone();

    let backend_workout = backend.clone();
    let workout = use_resource(move || {
        let wid = id_workout.clone();
        let backend = backend_workout.clone();
        let session = session_signal.read().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let rows = backend
                .get_workouts_by_ids(sess.access_token(), &[wid])
                .await?;
            Ok(rows.into_iter().next())
        }
    });

    let backend_exercises = backend.clone();
    let exercises = use_resource(move || {
        let wid = id_exercises.clone();
        let backend = backend_exercises.clone();
        let session = session_signal.read().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            backend
                .list_exercises_for_workout(sess.access_token(), &wid)
                .await
        }
    });

    let backend_library = backend.clone();
    let library = use_resource(move || {
        let backend = backend_library.clone();
        let session = session_signal.read().clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let specialist_id = sess.user_id().to_string();
            backend
                .list_exercise_library(sess.access_token(), &specialist_id, None)
                .await
        }
    });

    let mut add_exercise_id = use_signal(|| Option::<String>::None);
    let mut add_loading = use_signal(|| false);

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::LoginView {}, "Ir a login" } }
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
    let ex_ids_in_workout: std::collections::HashSet<String> =
        exs.iter().map(|e| e.id.clone()).collect();
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
            let backend_row = backend.clone();
            rsx! {
                li { key: "{eid}",
                    span { "{ename}" }
                    if deleted {
                        span { class: "text-xs text-text-muted", " (eliminado en biblioteca)" }
                    }
                    button {
                        class: "min-h-9 px-2 text-sm rounded-md border border-border",
                        onclick: move |_| {
                            let backend = backend_row.clone();
                            let sess = session_signal.read().clone();
                            let Some(s) = sess else { return };
                            let exercise_id = eid.clone();
                            let workout_id_for_remove = wid.clone();
                            spawn(async move {
                                let _ = backend.remove_exercise_from_workout(s.access_token(), &workout_id_for_remove, &exercise_id).await;
                                ex_refresh.restart();
                            });
                        },
                        "Quitar"
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "view container mx-auto workout-editor flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                h1 { class: "text-2xl font-semibold mb-4", "Entrenamiento" }
                nav { class: "flex flex-wrap gap-2 mb-6 pb-4 border-b border-border",
                    Link { to: Route::WorkoutLibrary {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100", "← Biblioteca de entrenamientos" }
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
                            p { class: "text-sm text-text-muted", "Aún no hay ejercicios. Añade desde la biblioteca abajo." }
                        }
                    }
                    section { class: "mt-6",
                        h3 { class: "text-lg font-semibold mb-2", "Añadir desde biblioteca de ejercicios" }
                        if available_to_add.is_empty() {
                            p { class: "text-sm text-text-muted", "Todos los ejercicios ya están en este entrenamiento o no hay ejercicios en la biblioteca." }
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
                                    let backend = backend.clone();
                                    let sess = session_signal.read().clone();
                                    let Some(s) = sess else { return };
                                    let wid = id.clone();
                                    add_loading.set(true);
                                    let mut ex_refresh = exercises;
                                    spawn(async move {
                                        let order = 0;
                                        let _ = backend.add_exercise_to_workout(s.access_token(), &wid, &eid, order).await;
                                        ex_refresh.restart();
                                        add_loading.set(false);
                                    });
                                },
                                "Añadir al entrenamiento"
                            }
                        }
                    }
                }
            }
        }
    }
}
