use dioxus::prelude::*;

use dioxus_i18n::t;

use crate::hooks::{
    add_exercise_to_workout::use_add_exercise_to_workout,
    remove_exercise_from_workout::use_remove_exercise_from_workout,
    update_workout_exercise::use_update_workout_exercise, workout_editor::use_workout_editor,
    AsyncState,
};
use application::ports::backoffice_api::WorkoutEditorExerciseItem;

#[component]
pub fn WorkoutEditor(id: String) -> Element {
    let data = use_workout_editor(id.clone());
    let add_exercise = use_add_exercise_to_workout();
    let remove_exercise = use_remove_exercise_from_workout();
    let update_exercise = use_update_workout_exercise();

    let mut add_exercise_id = use_signal(|| Option::<String>::None);
    let sets_reps = use_signal(|| std::collections::HashMap::<String, (i32, i32)>::new());

    let (workout_opt, exs, library_list) = match &*data.state.read() {
        AsyncState::Idle | AsyncState::Loading => (None, Vec::new(), Vec::new()),
        AsyncState::Error(_) => (None, Vec::new(), Vec::new()),
        AsyncState::Ready(d) => (d.workout.clone(), d.exercises.clone(), d.library.clone()),
    };
    let ex_ids_in_workout: std::collections::HashSet<String> =
        exs.iter().map(|we| we.exercise.id.clone()).collect();
    let available_to_add: Vec<&WorkoutEditorExerciseItem> = library_list
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
            let ex_refresh = data.resource.clone();
            let mut sets_reps_sig = sets_reps;
            let remove_ex = remove_exercise.action;
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
            let can_up = idx > 0;
            let can_down = idx < exs.len().saturating_sub(1);
            let (prev_id, prev_order, prev_sets, prev_reps) = if can_up {
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
            let (next_id, next_order, next_sets, next_reps) = if can_down {
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
            let update_ex_up = update_exercise.action;
            let update_ex_down = update_exercise.action;
            rsx! {
                li { key: "{eid}", class: "flex flex-wrap items-center gap-2 py-1",
                    if can_up {
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border",
                            title: t!("workout_editor_move_up"),
                            onclick: move |_| {
                                let wid2 = wid_up.clone();
                                let eid_cur = eid_subir.clone();
                                let eid_prev = prev_id.clone();
                                let mut action_up = update_ex_up.clone();
                                let mut action_prev = update_exercise.action.clone();
                                let mut ex_refresh = ex_refresh.clone();
                                async move {
                                    action_up.call((wid2.clone(), eid_cur.clone(), sets_initial, reps_initial, Some(prev_order))).await;
                                    action_prev.call((wid2, eid_prev, prev_sets, prev_reps, Some(my_order))).await;
                                    ex_refresh.restart();
                                }
                            },
                            {t!("workout_editor_up_arrow")}
                        }
                    }
                    if can_down {
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border",
                            title: t!("workout_editor_move_down"),
                            onclick: move |_| {
                                let wid2 = wid_down.clone();
                                let eid_cur = eid_bajar.clone();
                                let eid_next = next_id.clone();
                                let mut action_cur = update_ex_down.clone();
                                let mut action_next = update_exercise.action.clone();
                                let mut ex_refresh = ex_refresh.clone();
                                async move {
                                    action_cur.call((wid2.clone(), eid_cur.clone(), sets_initial, reps_initial, Some(next_order))).await;
                                    action_next.call((wid2, eid_next, next_sets, next_reps, Some(my_order))).await;
                                    ex_refresh.restart();
                                }
                            },
                            {t!("workout_editor_down_arrow")}
                        }
                    }
                    span { class: "font-medium", "{ename}" }
                    if deleted {
                        span { class: "text-xs text-text-muted", {t!("workout_editor_deleted_in_library")} }
                    }
                    span { class: "text-sm text-text-muted", {t!("workout_editor_series")} }
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
                            let eid2 = eid_sets.clone();
                            let wid2 = wid_sets.clone();
                            let mut action = update_exercise.action.clone();
                            let mut ex_refresh = ex_refresh.clone();
                            async move {
                                action.call((wid2, eid2, v, r, None)).await;
                                ex_refresh.restart();
                            }
                        },
                    }
                    span { class: "text-sm text-text-muted", {t!("workout_editor_reps")} }
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
                            let eid2 = eid_reps.clone();
                            let wid2 = wid_reps.clone();
                            let mut action = update_exercise.action.clone();
                            let mut ex_refresh = ex_refresh.clone();
                            async move {
                                action.call((wid2, eid2, s, v, None)).await;
                                ex_refresh.restart();
                            }
                        },
                    }
                    button {
                        class: "min-h-9 px-2 text-sm rounded-md border border-border",
                        onclick: move |_| {
                            let exercise_id = eid_remove.clone();
                            let workout_id_for_remove = wid_remove.clone();
                            let mut action = remove_ex.clone();
                            let mut ex_refresh = ex_refresh.clone();
                            async move {
                                action.call((workout_id_for_remove, exercise_id)).await;
                                ex_refresh.restart();
                            }
                        },
                        {t!("workout_editor_remove")}
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "view container mx-auto workout-editor w-full",
            div {
                class: "content w-full",
                if let Some(ref w) = workout_opt {
                    h2 { "{w.name}" }
                    if let Some(ref d) = w.description {
                        if !d.is_empty() {
                            p { "{d}" }
                        }
                    }
                } else if matches!(&*data.state.read(), AsyncState::Ready(_)) {
                    p { { t!("workout_editor_not_found") } }
                } else {
                    p { { t!("workout_editor_loading") } }
                }
                if workout_opt.is_some() {
                    section {
                        h3 { { t!("workout_editor_exercises_in_workout") } }
                        ul { class: "exercise-list",
                            {exercise_rows.into_iter()}
                        }
                        if exs.is_empty() {
                            p { class: "text-sm text-text-muted", { t!("workout_editor_no_exercises") } }
                        }
                    }
                    section { class: "mt-6",
                        h3 { class: "text-lg font-semibold mb-2", { t!("workout_editor_add_from_library") } }
                        if available_to_add.is_empty() {
                            p { class: "text-sm text-text-muted", { t!("workout_editor_all_added") } }
                        } else {
                            select {
                                onchange: move |ev| {
                                    let v = ev.value();
                                    add_exercise_id.set(if v.is_empty() { None } else { Some(v) });
                                },
                                option { value: "", { t!("workout_editor_select_exercise") } }
                                for exercise in available_to_add.iter() {
                                    option { value: "{exercise.id}", "{exercise.name}" }
                                }
                            }
                            button {
                                disabled: add_exercise.state.read().is_loading() || add_exercise_id().is_none(),
                                onclick: move |_| {
                                    let eid = match add_exercise_id() {
                                        Some(eid) => eid,
                                        None => return,
                                    };
                                    let wid = id.clone();
                                    let order_index = exs.len() as i32;
                                    let mut action = add_exercise.action.clone();
                                    let mut resource = data.resource.clone();
                                    let mut add_id_signal = add_exercise_id;
                                    spawn(async move {
                                        action.call((wid, eid, order_index, 3, 10)).await;
                                        resource.restart();
                                        add_id_signal.set(None);
                                    });
                                },
                                { t!("workout_editor_add_to_workout") }
                            }
                        }
                    }
                }
            }
        }
    }
}
