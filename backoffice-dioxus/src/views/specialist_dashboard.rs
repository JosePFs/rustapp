use std::collections::HashSet;

use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::hooks::{
    link_patient::use_link_patient, specialist_patients::use_specialist_patients,
    unassigned_patients::use_unassigned_patients, AsyncState,
};
use crate::Route;

#[component]
pub fn SpecialistPatients() -> Element {
    let nav = use_navigator();
    let patients = use_specialist_patients();
    let unassigned_patients = use_unassigned_patients();
    let link_patient = use_link_patient();
    let mut selected_patient_ids = use_signal(|| HashSet::<String>::new());
    let mut filter_linked = use_signal(|| String::new());
    let mut filter_unassigned = use_signal(|| String::new());

    if let Some(_) = patients.state.read().auth_error() {
        nav.push(Route::Login {});
        return rsx! {};
    }

    let patients_resource = patients.resource.clone();
    let unassigned_resource = unassigned_patients.resource.clone();

    rsx! {
        div {
            class: "view container mx-auto specialist-dashboard w-full",
            div { class: "content w-full",
                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h3 { class: "text-lg font-semibold mb-2", { t!("specialist_patients_title") } }
                    div { class: "flex gap-2 mt-0 mb-2",
                        p { class: "text-sm text-text-muted mb-4", { t!("specialist_dashboard_instructions") } }
                    }
                    input {
                        class: "w-full min-h-11 px-4 border border-border rounded-md mb-4 focus:outline-none focus:border-primary",
                        placeholder: "{t!(\"workout_library_filter_placeholder\")}",
                        value: "{filter_linked()}",
                        oninput: move |ev| filter_linked.set(ev.value().clone()),
                    }
                    {
                        match &*patients.state.read() {
                            AsyncState::Idle | AsyncState::Loading => rsx! {
                                p { { t!("loading_patients") } }
                            },
                            AsyncState::Error(_) => rsx! {
                                p { class: "text-error", { t!("error_load_patients") } }
                            },
                            AsyncState::Ready(data) => {
                                let filter = filter_linked().to_lowercase();
                                let filtered_links: Vec<_> = data.links.iter()
                                    .filter(|link| {
                                        if filter.is_empty() { return true; }
                                        if let Some(profile) = data.profiles.iter().find(|p| p.patient_id == link.patient_id) {
                                            profile.full_name.to_lowercase().contains(&filter) || profile.email.to_lowercase().contains(&filter)
                                        } else {
                                            false
                                        }
                                    })
                                    .collect();
                                if filtered_links.is_empty() {
                                    rsx! { p { class: "text-text-muted italic", { t!("specialist_dashboard_no_patients_found") } } }
                                } else {
                                    rsx! {
                                        ul { class: "list-none p-0 m-0 mb-4",
                                            for link in filtered_links {
                                                li { key: "{link.link_id}", class: "mb-1",
                                                    Link {
                                                        to: Route::PatientProgress { id: link.patient_id.clone() },
                                                        class: "block p-4 min-h-11 text-primary no-underline rounded-md border border-border bg-surface hover:bg-gray-50 hover:border-primary focus-ring",
                                                        {
                                                            data.profiles
                                                                .iter()
                                                                .find(|p| p.patient_id == link.patient_id)
                                                                .map(|p| rsx! { "{p.full_name} ({p.email})" })
                                                                .unwrap_or(rsx! { "{link.patient_id}" })
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h3 { class: "text-lg font-semibold mb-2", { t!("add_patient_existing") } }
                    p { class: "text-sm text-text-muted mb-4", { t!("specialist_dashboard_link_instructions") } }

                    if unassigned_patients.state.read().is_not_ready() {
                        p { class: "text-text-muted", { t!("loading") } }
                    } else {
                        match &*unassigned_patients.state.read() {
                            AsyncState::Idle | AsyncState::Loading => rsx! {
                                p { { t!("loading_unassigned_patients") } }
                            },
                            AsyncState::Error(_) => rsx! {
                                p { class: "text-error", { t!("error_load_unassigned_patients") } }
                            },
                            AsyncState::Ready(data) => {
                                let filter = filter_unassigned().to_lowercase();
                                let filtered_patients: Vec<_> = data.patients.iter()
                                    .filter(|p| {
                                        if filter.is_empty() { return true; }
                                        p.full_name.to_lowercase().contains(&filter) || p.email.to_lowercase().contains(&filter)
                                    })
                                    .collect();
                                if filtered_patients.is_empty() {
                                    rsx! { p { class: "text-text-muted italic", { t!("specialist_dashboard_no_unassigned_found") } } }
                                } else {
                                    let patients: Vec<_> = filtered_patients.iter().map(|p| (p.email.clone(), p.full_name.clone(), p.email.clone())).collect();
                                    rsx! {
                                        input {
                                            class: "w-full min-h-11 px-4 border border-border rounded-md mb-4 focus:outline-none focus:border-primary",
                                            placeholder: "{t!(\"workout_library_filter_placeholder\")}",
                                            value: "{filter_unassigned()}",
                                            oninput: move |ev| filter_unassigned.set(ev.value().clone()),
                                        }
                                        div { class: "max-h-64 overflow-y-auto border border-border rounded-md p-2 mb-4",
                                            for (email_key, full_name, email) in patients {
                                                label { class: "flex items-center gap-2 p-2 min-h-11 cursor-pointer rounded hover:bg-gray-50",
                                                    input { class: "w-5 h-5",
                                                        r#type: "checkbox",
                                                        checked: selected_patient_ids().contains(&email_key),
                                                        onchange: move |ev| {
                                                            let mut set = selected_patient_ids();
                                                            if ev.checked() {
                                                                set.insert(email_key.clone());
                                                            } else {
                                                                set.remove(&email_key);
                                                            }
                                                            selected_patient_ids.set(set);
                                                        },
                                                    }
                                                    span { "{full_name} ({email})" }
                                                }
                                            }
                                        }
                                        div { class: "flex gap-2",
                                            button {
                                                class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed flex items-center gap-2",
                                                disabled: link_patient.state.read().is_loading() || selected_patient_ids().is_empty(),
                                                onclick: move |_| {
                                                    let selected = selected_patient_ids();
                                                    if selected.is_empty() { return; }
                                                    let email = selected.iter().next().unwrap().clone();
                                                    let mut action = link_patient.action.clone();
                                                    let mut selected_ids = selected_patient_ids;
                                                    let mut patients_resource_clone = patients_resource.clone();
                                                    let mut unassigned_resource_clone = unassigned_resource.clone();
                                                    spawn(async move {
                                                        action.call(email).await;
                                                        patients_resource_clone.restart();
                                                        unassigned_resource_clone.restart();
                                                        selected_ids.set(HashSet::new());
                                                    });
                                                },
                                                { t!("link_selected_patients") }
                                            }
                                            if let Some(e) = link_patient.state.read().error() {
                                                p { class: "text-error text-sm", "{e}" }
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
