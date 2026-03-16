use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::app_context::AppContext;
use crate::hooks::exercise_library::use_exercise_library;
use crate::hooks::AsyncState;
use crate::Route;

#[component]
pub fn ExerciseLibrary() -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let mut filter = use_signal(|| String::new());
    let exercises = use_exercise_library(filter);

    let mut new_name = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut new_video_url = use_signal(|| String::new());
    let mut create_loading = use_signal(|| false);
    let mut create_error = use_signal(|| Option::<String>::None);
    let mut editing_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_desc = use_signal(|| String::new());
    let mut edit_video_url = use_signal(|| String::new());

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

    let (list, list_len, empty_ok) = match &*exercises.state.read() {
        AsyncState::Idle | AsyncState::Loading => (Vec::new(), 0, false),
        AsyncState::Error(_) => (Vec::new(), 0, false),
        AsyncState::Ready(data) => (data.clone(), data.len(), true),
    };
    let backend_for_rows = backend.clone();
    let backend_for_create = backend.clone();
    let session_for_create = session_signal.read().clone();
    let rows: Vec<Element> = list
        .into_iter()
        .map(|ex| {
            let backend_row = backend_for_rows.clone();
            let session_row = session_for_create.clone();
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
                                    let backend = backend_row.clone();
                                    let token = session_row.as_ref().map(|s| s.access_token().to_string()).unwrap_or_default();
                                    let eid = ex_id_edit.clone();
                                    let n = edit_name().clone();
                                    let d = edit_desc().clone();
                                    let v = edit_video_url().clone();
                                    editing_id.set(None);
                                    let mut resource = exercises.resource.clone();
                                    spawn(async move {
                                        let _ = backend.update_exercise(&token, &eid, Some(&n), Some(if d.is_empty() { "" } else { &d }), None, Some(if v.is_empty() { None } else { Some(v.as_str()) })).await;
                                        resource.restart();
                                    });
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
                                    let backend = backend_row.clone();
                                    let token = session_row.as_ref().map(|s| s.access_token().to_string()).unwrap_or_default();
                                    let eid = ex_id_del.clone();
                                    let mut resource = exercises.resource.clone();
                                    spawn(async move {
                                        let _ = backend.soft_delete_exercise(token.as_str(), &eid).await;
                                        resource.restart();
                                    });
                                },
                                { t!("exercise_library_delete") }
                            }
                        } else {
                            button {
                                class: "min-h-9 px-2 text-sm rounded-md border border-border mt-2",
                                onclick: move |_| {
                                    let backend = backend_row.clone();
                                    let token = session_row.as_ref().map(|s| s.access_token().to_string()).unwrap_or_default();
                                    let eid = ex_id_restore.clone();
                                    let mut resource = exercises.resource.clone();
                                    spawn(async move {
                                        let _ = backend.restore_exercise(token.as_str(), &eid).await;
                                        resource.restart();
                                    });
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
                    value: "{filter()}",
                    oninput: move |ev| filter.set(ev.value().clone()),
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
                            disabled: create_loading() || new_name().trim().is_empty(),
                            onclick: move |_| {
                                let name = new_name().trim().to_string();
                                if name.is_empty() { return; }
                                let backend = backend_for_create.clone();
                                let session = session_for_create.clone();
                                let token = session.as_ref().map(|s| s.access_token().to_string()).unwrap_or_default();
                                let specialist_id = session.as_ref().map(|s| s.user_id().to_string()).unwrap_or_default();
                                let desc = new_desc().clone();
                                let video = new_video_url().clone();
                                create_loading.set(true);
                                create_error.set(None);
                                let mut resource = exercises.resource.clone();
                                spawn(async move {
                                    match backend.create_exercise(
                                        &token,
                                        &specialist_id,
                                        &name,
                                        if desc.is_empty() { None } else { Some(desc.as_str()) },
                                        0,
                                        if video.is_empty() { None } else { Some(video.as_str()) },
                                    ).await {
                                        Ok(_) => {
                                            new_name.set(String::new());
                                            new_desc.set(String::new());
                                            new_video_url.set(String::new());
                                            resource.restart();
                                        }
                                        Err(e) => create_error.set(Some(e.to_string())),
                                    }
                                    create_loading.set(false);
                                });
                            },
                            { t!("exercise_library_create_btn") }
                        }
                        if let Some(ref e) = *create_error.read() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-4", { t!("exercise_library_list_title", count: list_len.to_string()) } }
                    ul { class: "list-none p-0 m-0",
                        {rows.into_iter()}
                    }
                    if list_len == 0 && empty_ok {
                        p { class: "text-text-muted italic py-4", { t!("exercise_library_empty") } }
                    }
                }
            }
        }
    }
}
