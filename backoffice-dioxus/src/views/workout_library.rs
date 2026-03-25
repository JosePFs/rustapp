use dioxus::prelude::*;

use dioxus_free_icons::icons::io_icons::{IoAdd, IoClose, IoEye, IoPencil, IoSave, IoTrash};
use dioxus_free_icons::Icon;
use dioxus_i18n::t;
use dioxus_router::Link;

use crate::hooks::{
    create_workout::use_create_workout, delete_workout::use_delete_workout,
    update_workout::use_update_workout, workout_library::use_workout_library, AsyncState,
};
use crate::Route;
use application::use_cases::update_workout::UpdateWorkoutInput;

#[component]
pub fn WorkoutLibrary() -> Element {
    let mut filter = use_signal(|| String::new());
    let workouts = use_workout_library(filter);
    let update_workout = use_update_workout();
    let create_workout = use_create_workout();
    let delete_workout = use_delete_workout();

    let mut new_name = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_desc = use_signal(|| String::new());

    let (list, list_len, empty_ok) = match &*workouts.state.read() {
        AsyncState::Idle | AsyncState::Loading => (Vec::new(), 0, false),
        AsyncState::Error(_) => (Vec::new(), 0, false),
        AsyncState::Ready(data) => (data.clone(), data.len(), true),
    };
    let rows: Vec<Element> = list
        .into_iter()
        .map(|wo| {
            let wid = wo.id.clone();
            let wid_edit = wo.id.clone();
            let wid_del = wo.id.clone();
            let name = wo.name.clone();
            let desc = wo.description.clone().unwrap_or_default();
            rsx! {
                li { key: "{wid}", class: "p-4 bg-surface border border-border rounded-md mb-2",
                    if editing_id().as_ref() == Some(&wid_edit) {
                        div { class: "flex flex-wrap gap-2 items-center mt-2",
                            input {
                                class: "flex-1 min-w-32 min-h-9 px-3 border border-border rounded-md text-sm",
                                placeholder: "{t!(\"workout_library_name_placeholder\")}",
                                value: "{edit_name()}",
                                oninput: move |ev| edit_name.set(ev.value().clone()),
                            }
                            input {
                                class: "flex-1 min-w-32 min-h-9 px-3 border border-border rounded-md text-sm",
                                placeholder: "{t!(\"workout_library_desc_placeholder\")}",
                                value: "{edit_desc()}",
                                oninput: move |ev| edit_desc.set(ev.value().clone()),
                            }
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md bg-primary text-white focus-ring flex items-center gap-1",
                                onclick: move |_| {
                                    let id = wid_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    let mut action = update_workout.action.clone();
                                    let mut resource = workouts.resource.clone();
                                    editing_id.set(None);
                                    async move {
                                        action.call(UpdateWorkoutInput {
                                            workout_id: id,
                                            name: n,
                                            description: d,
                                        }).await;
                                        resource.restart();
                                    }
                                },
                                Icon { width: 14, height: 14, icon: IoSave }
                                { t!("workout_library_save") }
                            }
                            button { class: "min-h-9 px-2 text-sm rounded-md border border-border focus-ring flex items-center gap-1", onclick: move |_| editing_id.set(None),
                                Icon { width: 14, height: 14, icon: IoClose }
                                { t!("workout_library_cancel") }
                            }
                        }
                    } else {
                        span { class: "block",
                            strong { "{name}" }
                            if !desc.is_empty() { span { " — {desc}" } }
                        }
                        Link {
                            to: Route::WorkoutEditor { id: wid.clone() },
                            class: "inline-block min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2 text-primary no-underline hover:bg-gray-50 focus-ring flex items-center gap-1",
                            Icon { width: 14, height: 14, icon: IoEye }
                            { t!("workout_library_exercises_link") }
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2 focus-ring",
                            onclick: move |_| {
                                edit_name.set(name.clone());
                                edit_desc.set(desc.clone());
                                editing_id.set(Some(wid_edit.clone()));
                            },
                            Icon { width: 14, height: 14, icon: IoPencil }
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md bg-error text-white mt-2 focus-ring",
                            onclick: move |_| {
                                let id = wid_del.clone();
                                let mut action = delete_workout.action.clone();
                                let mut resource = workouts.resource.clone();
                                async move {
                                    action.call((id,)).await;
                                    resource.restart();
                                }
                            },
                            Icon { width: 14, height: 14, icon: IoTrash }
                        }
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "view container mx-auto workout-library w-full",
            div { class: "content w-full",
                p { class: "text-sm text-text-muted mb-4", { t!("workout_library_intro") } }
                input {
                    class: "w-full min-h-11 px-4 border border-border rounded-md mb-4 focus:outline-none focus:border-primary",
                    placeholder: "{t!(\"workout_library_filter_placeholder\")}",
                    value: "{filter()}",
                    oninput: move |ev| filter.set(ev.value().clone()),
                }
                section { class: "bg-surface rounded-lg p-4 mb-6 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", { t!("workout_library_new_section") } }
                    div { class: "flex flex-col gap-4 max-w-md",
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "{t!(\"workout_library_name_placeholder\")}",
                            value: "{new_name()}",
                            oninput: move |ev| new_name.set(ev.value().clone()),
                        }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "{t!(\"workout_library_desc_placeholder\")}",
                            value: "{new_desc()}",
                            oninput: move |ev| new_desc.set(ev.value().clone()),
                        }
                        button {
                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 flex items-center gap-2",
                            disabled: create_workout.state.read().is_loading() || new_name().trim().is_empty(),
                            onclick: move |_| {
                                let name = new_name().trim().to_string();
                                if name.is_empty() { return; }
                                let desc = new_desc().clone();
                                let mut action = create_workout.action.clone();
                                let mut resource = workouts.resource.clone();
                                spawn(async move {
                                    action.call(name, desc).await;
                                    new_name.set(String::new());
                                    new_desc.set(String::new());
                                    resource.restart();
                                });
                            },
                            Icon { width: 18, height: 18, icon: IoAdd }
                            { t!("workout_library_create_btn") }
                        }
                        if let Some(e) = create_workout.state.read().error() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", { t!("workout_library_list_title", count: list_len.to_string()) } }
                    ul { class: "list-none p-0 m-0",
                        {rows.into_iter()}
                    }
                    if list_len == 0 && empty_ok {
                        p { class: "text-text-muted italic py-4", { t!("workout_library_empty") } }
                    }
                }
            }
        }
    }
}
