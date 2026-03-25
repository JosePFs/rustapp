use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::hooks::create_exercise::use_create_exercise;
use crate::hooks::exercise_library::use_exercise_library;
use crate::hooks::restore_exercise::use_restore_exercise;
use crate::hooks::soft_delete_exercise::use_soft_delete_exercise;
use crate::hooks::update_exercise::use_update_exercise;
use crate::Route;

#[component]
pub fn ExerciseLibrary() -> Element {
    let mut exercises = use_exercise_library();
    let create_exercise = use_create_exercise();
    let update_exercise = use_update_exercise();
    let soft_delete_exercise = use_soft_delete_exercise();
    let restore_exercise = use_restore_exercise();

    let mut new_name = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut new_video_url = use_signal(|| String::new());
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_desc = use_signal(|| String::new());
    let mut edit_video_url = use_signal(|| String::new());

    if let Some(e) = exercises.state.read().auth_error() {
        return Err(e.clone().into());
    }
    let (list, list_len, empty_ready) = exercises
        .state
        .read()
        .data()
        .map(|data| (data.clone(), data.len(), true))
        .unwrap_or_default();

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
                    class: if is_deleted { "opacity-65 p-4 bg-surface border border-border rounded-md mb-2" } else { "p-4 bg-surface border border-border rounded-md mb-2" },
                    if editing_id().as_ref() == Some(&ex_id_edit) {
                        div { class: "flex flex-wrap gap-2 items-center mt-2",
                            input {
                                class: "flex-1 min-w-24 min-h-9 px-3 border border-border rounded-md text-sm",
                                value: "{edit_name()}",
                                oninput: move |ev| edit_name.set(ev.value().clone()),
                            }
                            input {
                                class: "flex-1 min-w-24 min-h-9 px-3 border border-border rounded-md text-sm",
                                value: "{edit_desc()}",
                                oninput: move |ev| edit_desc.set(ev.value().clone()),
                            }
                            input {
                                class: "flex-1 min-w-24 min-h-9 px-3 border border-border rounded-md text-sm",
                                value: "{edit_video_url()}",
                                oninput: move |ev| edit_video_url.set(ev.value().clone()),
                            }
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md bg-primary text-white",
                                onclick: move |_| {
                                    let eid = ex_id_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    let v = edit_video_url().clone();
                                    let mut action = update_exercise.action.clone();
                                    let mut resource = exercises.resource.clone();
                                    editing_id.set(None);
                                    async move {
                                        action.call((eid, n, d, v)).await;
                                        resource.restart();
                                    }
                                },
                                { t!("exercise_library_save") }
                            }
                            button { class: "min-h-9 px-2 text-sm rounded-md border border-border", onclick: move |_| editing_id.set(None), { t!("exercise_library_cancel") } }
                        }
                    } else {
                        span { class: "block",
                            strong { "{name}" }
                            if !desc.is_empty() { span { " — {desc}" } }
                            if is_deleted {
                                span { class: "text-xs text-text-muted", { t!("exercise_library_deleted_label") } }
                            }
                        }
                        if !is_deleted {
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2",
                                onclick: move |_| {
                                    edit_name.set(name.clone());
                                    edit_desc.set(desc.clone());
                                    edit_video_url.set(video.clone());
                                    editing_id.set(Some(ex_id_edit.clone()));
                                },
                                { t!("exercise_library_edit") }
                            }
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md bg-error text-white mt-2 mr-2",
                                onclick: move |_| {
                                    let eid = ex_id_del.clone();
                                    let mut action = soft_delete_exercise.action.clone();
                                    let mut resource = exercises.resource.clone();
                                    async move {
                                        action.call((eid,)).await;
                                        resource.restart();
                                    }
                                },
                                { t!("exercise_library_delete") }
                            }
                        } else {
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2",
                                onclick: move |_| {
                                    let eid = ex_id_restore.clone();
                                    let mut action = restore_exercise.action.clone();
                                    let mut resource = exercises.resource.clone();
                                    async move {
                                        action.call((eid,)).await;
                                        resource.restart();
                                    }
                                },
                                { t!("exercise_library_restore") }
                            }
                        }
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div {
            class: "view container mx-auto exercise-library",
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
                                span { { t!("exercise_library_title") } }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPatients {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", { t!("exercise_library_nav_patients") } }
                                }
                            }
                        }
                    }
                }
                p { class: "text-sm text-text-muted mb-4", { t!("exercise_library_intro") } }
                input {
                    class: "w-full min-h-11 px-4 border border-border rounded-md mb-4 focus:outline-none focus:border-primary",
                    placeholder: "{t!(\"exercise_library_filter_placeholder\")}",
                    value: exercises.filter.read().clone(),
                    oninput: move |ev| exercises.filter.set(ev.value().clone()),
                }
                section { class: "bg-surface rounded-lg p-4 mb-6 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", { t!("exercise_library_new_section") } }
                    div { class: "flex flex-col gap-4 max-w-md",
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "{t!(\"exercise_library_name_placeholder\")}",
                            value: "{new_name()}",
                            oninput: move |ev| new_name.set(ev.value().clone()),
                        }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "{t!(\"exercise_library_desc_placeholder\")}",
                            value: "{new_desc()}",
                            oninput: move |ev| new_desc.set(ev.value().clone()),
                        }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md focus:outline-none focus:border-primary",
                            placeholder: "{t!(\"exercise_library_video_placeholder\")}",
                            value: "{new_video_url()}",
                            oninput: move |ev| new_video_url.set(ev.value().clone()),
                        }
                        button {
                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                            disabled: create_exercise.state.read().is_loading() || new_name().trim().is_empty(),
                            onclick: move |_| {
                                let name = new_name().trim().to_string();
                                if name.is_empty() { return; }
                                let desc = new_desc().clone();
                                let video = if new_video_url().is_empty() { None } else { Some(new_video_url().clone()) };
                                let mut action = create_exercise.action.clone();
                                let mut resource = exercises.resource.clone();
                                spawn(async move {
                                    action.call((name, desc, 0, video)).await;
                                    new_name.set(String::new());
                                    new_desc.set(String::new());
                                    new_video_url.set(String::new());
                                    resource.restart();
                                });
                            },
                            { t!("exercise_library_create_btn") }
                        }
                        if let Some(e) = create_exercise.state.read().error() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", { t!("exercise_library_list_title", count: list_len.to_string()) } }
                    ul { class: "list-none p-0 m-0",
                        {rows.into_iter()}
                    }
                    if list_len == 0 && empty_ready {
                        p { class: "text-text-muted italic py-4", { t!("exercise_library_empty") } }
                    }
                }
            }
        }
    }
}
