use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::app_context::AppContext;
use crate::hooks::{workout_library::use_workout_library, AsyncState};
use crate::Route;

#[component]
pub fn WorkoutLibrary() -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let mut filter = use_signal(|| String::new());
    let workouts = use_workout_library(filter);

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
            div {
                { t!("must_login_message") }
                " "
                Link { to: Route::LoginView {}, { t!("go_to_login") } }
            }
        };
    }

    let (list, list_len, empty_ok) = match &*workouts.state.read() {
        AsyncState::Idle | AsyncState::Loading => (Vec::new(), 0, false),
        AsyncState::Error(_) => (Vec::new(), 0, false),
        AsyncState::Ready(data) => (data.clone(), data.len(), true),
    };
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
                                    let mut resource = workouts.resource.clone();
                                    spawn(async move {
                                        let _ = backend.update_workout(&token, &id, Some(&n), Some(if d.is_empty() { None } else { Some(d.as_str()) }), None).await;
                                        resource.restart();
                                    });
                                },
                                { t!("workout_library_save") }
                            }
                            button { class: "min-h-9 px-2 text-sm rounded-md border border-border", onclick: move |_| editing_id.set(None), { t!("workout_library_cancel") } }
                        }
                    } else {
                        span { class: "block",
                            strong { "{name}" }
                            if !desc.is_empty() { span { " — {desc}" } }
                        }
                        Link {
                            to: Route::WorkoutEditor { id: wid.clone() },
                            class: "inline-block min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2 text-primary no-underline hover:bg-gray-50",
                            { t!("workout_library_exercises_link") }
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2 mr-2",
                            onclick: move |_| {
                                edit_name.set(name.clone());
                                edit_desc.set(desc.clone());
                                editing_id.set(Some(wid_edit.clone()));
                            },
                            { t!("workout_library_edit") }
                        }
                        button {
                            class: "min-h-9 px-2 text-sm rounded-md bg-error text-white mt-2",
                            onclick: move |_| {
                                let backend = backend_row.clone();
                                let sess = session_signal.read().clone();
                                let Some(s) = sess else { return };
                                let id = wid_del.clone();
                                let mut resource = workouts.resource.clone();
                                spawn(async move {
                                    let _ = backend.delete_workout(s.access_token(), &id).await;
                                    resource.restart();
                                });
                            },
                            { t!("workout_library_delete") }
                        }
                    }
                }
            }
            .into()
        })
        .collect();

    rsx! {
        div { class: "view container mx-auto workout-library",
            div {
                class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                {
                    let mut nav_open = use_signal(|| false);
                    rsx! {
                        nav { class: "relative mb-6",
                            button {
                                class: "min-h-11 px-0 bg-transparent text-2xl font-semibold inline-flex items-center gap-2 text-text",
                                onclick: move |_| nav_open.set(!nav_open()),
                                span { { t!("workout_library_title") } }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPatients {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", { t!("workout_library_nav_patients") } }
                                    Link { to: Route::ExerciseLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", { t!("workout_library_nav_exercises") } }
                                }
                            }
                        }
                    }
                }
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
                                let mut resource = workouts.resource.clone();
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
                                            resource.restart();
                                        }
                                        Err(e) => create_error.set(Some(e.to_string())),
                                    }
                                    create_loading.set(false);
                                });
                            },
                            { t!("workout_library_create_btn") }
                        }
                        if let Some(ref e) = *create_error.read() {
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
