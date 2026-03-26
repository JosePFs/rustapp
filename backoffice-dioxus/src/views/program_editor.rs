use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::hooks::create_program_schedule_item::use_create_program_schedule_item;
use crate::hooks::delete_program_schedule_item::use_delete_program_schedule_item;
use crate::hooks::list_program_schedule::use_program_schedule_data;
use crate::hooks::workout_library_data::use_workout_library_data;

#[component]
pub fn ProgramEditor(id: String) -> Element {
    let program_id = id.clone();

    let create_schedule_item = use_create_program_schedule_item();
    let delete_schedule_item = use_delete_program_schedule_item();
    let schedule_data = use_program_schedule_data(program_id.clone());
    let library_workouts = use_workout_library_data();

    let mut schedule_block_rest = use_signal(|| true);
    let mut schedule_workout_id = use_signal(|| Option::<String>::None);
    let mut schedule_days = use_signal(|| 1i32);

    let schedule_data_value = schedule_data
        .resource
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned());

    let (schedule_items, schedule_workouts) = schedule_data_value
        .map(|d| (d.schedule, d.workouts))
        .unwrap_or((vec![], vec![]));
    let workout_names: HashMap<String, String> = schedule_workouts
        .iter()
        .map(|w| (w.id.clone(), w.name.clone()))
        .collect();
    let library_list = library_workouts
        .resource
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
                    .unwrap_or_else(|| t!("rest_day_label").to_string());
                let days = item.days_count;
                let item_id = item.id.clone();
                let mut delete_action = delete_schedule_item.action.clone();
                rsx! {
                    li { key: "{item_id}", class: "flex items-center gap-2 py-1 border-b border-border",
                        span { class: "font-medium", "{label}" }
                        span { class: "text-sm text-text-muted", "{days} " }
                        span { class: "text-sm text-text-muted", {t!("days_count")} }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border bg-error text-white ml-auto",
                            onclick: move |_| {
                                let id = item_id.clone();
                                async move {
                                    delete_action.call((id,)).await;
                                }   
                            },
                            { t!("delete_action") }
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
                    { t!("schedule_rest_days") }
                }
                label { style: "display: flex; align-items: center; gap: 0.5rem;",
                    input {
                        r#type: "radio",
                        name: "schedule_type",
                        checked: !is_rest,
                        onchange: move |_| schedule_block_rest.set(false)
                    }
                    { t!("schedule_training_days") }
                }
                if !is_rest {
                    select {
                        onchange: move |ev| {
                            let v = ev.value();
                            schedule_workout_id.set(if v.is_empty() { None } else { Some(v) });
                        },
                        option { value: "", { t!("select_workout") } }
                        for workout in library_list.iter() {
                            option { value: "{workout.id}", "{workout.name}" }
                        }
                    }
                }
                div { style: "display: flex; align-items: center; gap: 0.5rem;",
                    label { { t!("days_label") } }
                    input {
                        r#type: "number",
                        min: "1",
                        value: "{days_val}",
                        oninput: move |ev| schedule_days.set(ev.value().parse().unwrap_or(1).max(1)),
                    }
                }
                button {
                    class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                    disabled: create_schedule_item.state.read().is_loading() || (!is_rest && schedule_workout_id().is_none()),
                    onclick: move |_| {
                        let pid = program_id.clone();
                        let rest = schedule_block_rest();
                        let wid = schedule_workout_id();
                        let days = schedule_days().max(1);
                        let order = schedule_items.len() as i32;
                        let mut action = create_schedule_item.action.clone();
                        let mut schedule_block_rest = schedule_block_rest;
                        let mut schedule_workout_id = schedule_workout_id;
                        let mut schedule_days = schedule_days;
                        async move {
                            let w = if rest { None } else { wid };
                            action.call((pid, order, w, days)).await;
                            schedule_block_rest.set(true);
                            schedule_workout_id.set(None);
                            schedule_days.set(1);
                        }
                    },
                    { t!("add_block") }
                }
            }
            if let Some(e) = create_schedule_item.state.read().error() {
                p { class: "text-error text-sm mt-2", "{e}" }
            }
        }
    };

    rsx! {
        div { class: "view container mx-auto program-editor w-full",
            div {
                class: "content w-full",
                section { class: "bg-surface rounded-lg p-4 mb-6 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", { t!("program_schedule_title") } }
                    p { class: "text-sm text-text-muted mb-4", { t!("program_schedule_description") } }
                    {schedule_section}
                }
            }
        }
    }
}
