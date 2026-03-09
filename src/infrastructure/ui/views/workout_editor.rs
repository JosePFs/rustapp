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
    let sets_reps = use_signal(|| std::collections::HashMap::<String, (i32, i32)>::new());

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
        exs.iter().map(|we| we.exercise.id.clone()).collect();
    let available_to_add: Vec<&Exercise> = library_list
        .iter()
        .filter(|e| !ex_ids_in_workout.contains(&e.id) && e.deleted_at.is_none())
        .collect();

    let exercise_rows: Vec<Element> = exs
        .iter()
        .enumerate()
        .map(|(idx, we)| {
            let e = &we.exercise;
            let eid = e.id.clone();
            let ename = e.name.clone();
            let deleted = e.deleted_at.is_some();
            let (sets_val, reps_val) = sets_reps().get(&eid).copied().unwrap_or((we.sets, we.reps));
            let wid = id.clone();
            let ex_refresh = exercises.clone();
            let mut sets_reps_sig = sets_reps;
            let backend_sets = backend.clone();
            let backend_reps = backend.clone();
            let backend_remove = backend.clone();
            let backend_up = backend.clone();
            let backend_down = backend.clone();
            let sess_sig = session_signal.clone();
            let eid_sets = eid.clone();
            let eid_reps = eid.clone();
            let eid_remove = eid.clone();
            let wid_sets = wid.clone();
            let wid_reps = wid.clone();
            let wid_remove = wid.clone();
            let wid_up = wid.clone();
            let wid_down = wid.clone();
            let sets_initial = we.sets;
            let reps_initial = we.reps;
            let can_subir = idx > 0;
            let can_bajar = idx < exs.len().saturating_sub(1);
            let (prev_id, prev_order, prev_sets, prev_reps) = if can_subir {
                let prev = &exs[idx - 1];
                (
                    prev.exercise.id.clone(),
                    prev.order_index,
                    prev.sets,
                    prev.reps,
                )
            } else {
                (String::new(), 0, 0, 0)
            };
            let (next_id, next_order, next_sets, next_reps) = if can_bajar {
                let next = &exs[idx + 1];
                (
                    next.exercise.id.clone(),
                    next.order_index,
                    next.sets,
                    next.reps,
                )
            } else {
                (String::new(), 0, 0, 0)
            };
            let my_order = we.order_index;
            let eid_subir = eid.clone();
            let eid_bajar = eid.clone();
            rsx! {
                li { key: "{eid}", class: "flex flex-wrap items-center gap-2 py-1",
                    if can_subir {
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border",
                            title: "Subir",
                            onclick: move |_| {
                                let backend = backend_up.clone();
                                let sess = sess_sig.read().clone();
                                let Some(s) = sess else { return };
                                let wid2 = wid_up.clone();
                                let eid_cur = eid_subir.clone();
                                let eid_prev = prev_id.clone();
                                let mut ex_refresh = ex_refresh.clone();
                                spawn(async move {
                                    let _ = backend.update_workout_exercise(s.access_token(), &wid2, &eid_cur, sets_initial, reps_initial, Some(prev_order)).await;
                                    let _ = backend.update_workout_exercise(s.access_token(), &wid2, &eid_prev, prev_sets, prev_reps, Some(my_order)).await;
                                    ex_refresh.restart();
                                });
                            },
                            "↑"
                        }
                    }
                    if can_bajar {
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border",
                            title: "Bajar",
                            onclick: move |_| {
                                let backend = backend_down.clone();
                                let sess = sess_sig.read().clone();
                                let Some(s) = sess else { return };
                                let wid2 = wid_down.clone();
                                let eid_cur = eid_bajar.clone();
                                let eid_next = next_id.clone();
                                let mut ex_refresh = ex_refresh.clone();
                                spawn(async move {
                                    let _ = backend.update_workout_exercise(s.access_token(), &wid2, &eid_cur, sets_initial, reps_initial, Some(next_order)).await;
                                    let _ = backend.update_workout_exercise(s.access_token(), &wid2, &eid_next, next_sets, next_reps, Some(my_order)).await;
                                    ex_refresh.restart();
                                });
                            },
                            "↓"
                        }
                    }
                    span { class: "font-medium", "{ename}" }
                    if deleted {
                        span { class: "text-xs text-text-muted", " (eliminado en biblioteca)" }
                    }
                    span { class: "text-sm text-text-muted", "Series:" }
                    input {
                        class: "w-14 min-h-9 px-2 text-sm border border-border rounded",
                        r#type: "number",
                        min: "1",
                        value: "{sets_val}",
                        oninput: move |ev| {
                            let v = ev.value().parse().unwrap_or(1).max(1);
                            let (_, r) = sets_reps_sig().get(&eid_sets).copied().unwrap_or((sets_initial, reps_initial));
                            let mut m = sets_reps_sig();
                            m.insert(eid_sets.clone(), (v, r));
                            sets_reps_sig.set(m);
                            let backend = backend_sets.clone();
                            let sess = sess_sig.read().clone();
                            let Some(s) = sess else { return };
                            let eid2 = eid_sets.clone();
                            let wid2 = wid_sets.clone();
                            let mut ex_refresh = ex_refresh.clone();
                            spawn(async move {
                                let _ = backend.update_workout_exercise(s.access_token(), &wid2, &eid2, v, r, None).await;
                                ex_refresh.restart();
                            });
                        },
                    }
                    span { class: "text-sm text-text-muted", "Reps:" }
                    input {
                        class: "w-14 min-h-9 px-2 text-sm border border-border rounded",
                        r#type: "number",
                        min: "1",
                        value: "{reps_val}",
                        oninput: move |ev| {
                            let v = ev.value().parse().unwrap_or(1).max(1);
                            let (s, _) = sets_reps_sig().get(&eid_reps).copied().unwrap_or((sets_initial, reps_initial));
                            let mut m = sets_reps_sig();
                            m.insert(eid_reps.clone(), (s, v));
                            sets_reps_sig.set(m);
                            let backend = backend_reps.clone();
                            let sess = sess_sig.read().clone();
                            let Some(sess) = sess else { return };
                            let eid2 = eid_reps.clone();
                            let wid2 = wid_reps.clone();
                            let mut ex_refresh = ex_refresh.clone();
                            spawn(async move {
                                let _ = backend.update_workout_exercise(sess.access_token(), &wid2, &eid2, s, v, None).await;
                                ex_refresh.restart();
                            });
                        },
                    }
                    button {
                        class: "min-h-9 px-2 text-sm rounded-md border border-border",
                        onclick: move |_| {
                            let backend = backend_remove.clone();
                            let sess = sess_sig.read().clone();
                            let Some(s) = sess else { return };
                            let exercise_id = eid_remove.clone();
                            let workout_id_for_remove = wid_remove.clone();
                            let mut ex_refresh = ex_refresh.clone();
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
                {
                    let mut nav_open = use_signal(|| false);
                    rsx! {
                        nav { class: "relative mb-6 pb-4 border-b border-border",
                            button {
                                class: "min-h-11 px-3 rounded-md border border-border bg-surface hover:bg-gray-100 text-sm font-medium inline-flex items-center gap-1",
                                onclick: move |_| nav_open.set(!nav_open()),
                                span { "Menú" }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::WorkoutLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de entrenamientos" }
                                }
                            }
                        }
                    }
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
                                    let order_index = exs.len() as i32;
                                    add_loading.set(true);
                                    let mut ex_refresh = exercises.clone();
                                    let mut add_id_signal = add_exercise_id;
                                    spawn(async move {
                                        let _ = backend.add_exercise_to_workout(s.access_token(), &wid, &eid, order_index, 3, 10).await;
                                        ex_refresh.restart();
                                        add_id_signal.set(None);
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
