use std::collections::HashSet;

use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::Link;

use crate::hooks::{
    assign_program_to_patient::use_assign_program_to_patient, create_program::use_create_program,
    specialist_programs::use_specialist_programs, AsyncState,
};
use crate::Route;

#[component]
pub fn SpecialistPrograms() -> Element {
    let data = use_specialist_programs();
    let create_program = use_create_program();
    let assign_program = use_assign_program_to_patient();

    let mut new_program_name = use_signal(|| String::new());
    let mut new_program_desc = use_signal(|| String::new());

    let mut program_filter = use_signal(|| String::new());
    let mut patient_filter = use_signal(|| String::new());
    let mut selected_program_ids = use_signal(|| HashSet::<String>::new());
    let mut selected_patient_ids = use_signal(|| HashSet::<String>::new());

    rsx! {
        div {
            class: "view container mx-auto specialist-dashboard",
            div {
                class: "content w-full",

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    div { class: "flex gap-2 mt-0 mb-2",
                        p { class: "text-sm text-text-muted mb-4", { t!("specialist_programs_create_tooltip_1") } }
                        p { class: "text-sm text-text-muted mb-4", { t!("specialist_programs_create_tooltip_2") } }
                    }
                    {
                        match &*data.state.read() {
                            AsyncState::Idle | AsyncState::Loading => rsx! {
                                p { { t!("loading_programs") } }
                            },
                            AsyncState::Error(_) => rsx! {
                                p { class: "text-error", { t!("error_programs") } }
                            },
                            AsyncState::Ready(d) => rsx! {
                                ul { class: "list-none p-0 m-0 mb-4",
                                    for p in d.programs.iter() {
                                        li { key: "{p.id}", class: "mb-1",
                                            Link { to: Route::ProgramEditor { id: p.id.clone() }, class: "text-primary no-underline hover:underline focus-ring", "{p.name}" }
                                        }
                                    }
                                }
                            },
                        }
                    }
                    div { class: "flex flex-col gap-4 mt-4",
                        label { class: "text-sm font-medium text-text", { t!("program_name_label") } }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                            placeholder:  t!("program_name"),
                            r#type: "text",
                            value: "{new_program_name()}",
                            oninput: move |ev| new_program_name.set(ev.value().clone()),
                        }
                        label { class: "text-sm font-medium text-text", { t!("description_label") } }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                            placeholder:  t!("program_description"),
                            r#type: "text",
                            value: "{new_program_desc()}",
                            oninput: move |ev| new_program_desc.set(ev.value().clone()),
                        }
                        button {
                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                            disabled: create_program.state.read().is_loading() || new_program_name().trim().is_empty(),
                            onclick: move |_| {
                                let name = new_program_name().trim().to_string();
                                if name.is_empty() { return; }
                                let desc = new_program_desc().clone();
                                let mut action = create_program.action.clone();
                                let mut resource = data.resource.clone();
                                spawn(async move {
                                    action.call(name, desc).await;
                                    new_program_name.set(String::new());
                                    new_program_desc.set(String::new());
                                    resource.restart();
                                });
                            },
                            { t!("specialist_programs_create_btn") }
                        }
                        if let Some(e) = create_program.state.read().error() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", { t!("specialist_programs_assign_section") } }
                    {
                        match &*data.state.read() {
                            AsyncState::Idle | AsyncState::Loading => rsx! {
                                p { { t!("loading_programs") } }
                            },
                            AsyncState::Error(_) => rsx! {
                                p { class: "text-error", { t!("error_programs") } }
                            },
                            AsyncState::Ready(d) => {
                                let progs = d.programs.clone();
                                let assigns = d.assignments.clone();
                                let links = d.links.clone();
                                let profiles = d.profiles.clone();
                                rsx! {
                        p { class: "text-sm text-text-muted mb-4",
                            { t!("specialist_programs_assign_instructions") }
                        }

                        div { class: "mb-6",
                            h3 { class: "text-lg font-semibold mb-2", { t!("specialist_programs_programs_section") } }
                            label { class: "text-sm font-medium text-text", { t!("filter_programs_label") } }
                            input {
                                class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface mb-4 focus:outline-none focus:border-primary",
                                r#type: "text",
                                placeholder:  t!("specialist_programs_filter_programs_by_name"),
                                value: "{program_filter()}",
                                oninput: move |ev| program_filter.set(ev.value().clone()),
                            }
                            {
                                let filter = program_filter().to_lowercase();
                                let selected = selected_program_ids();
                                let rows: Vec<dioxus::prelude::Element> = progs
                                    .iter()
                                    .filter(|p| {
                                        filter.is_empty()
                                            || p.name.to_lowercase().contains(&filter)
                                    })
                                    .map(|p| {
                                        let pid = p.id.clone();
                                        let name = p.name.clone();
                                        let mut sel = selected_program_ids;
                                        let is_checked = selected.contains(&pid);
                                        rsx! {
                                            label { class: "flex items-center gap-2 p-2 min-h-11 cursor-pointer rounded hover:bg-gray-50",
                                                input { class: "w-5 h-5",
                                                    r#type: "checkbox",
                                                    checked: is_checked,
                                                    onchange: move |ev| {
                                                        let mut set = sel().clone();
                                                        if ev.checked() {
                                                            set.insert(pid.clone());
                                                        } else {
                                                            set.remove(&pid);
                                                        }
                                                        sel.set(set);
                                                    },
                                                }
                                                span { "{name}" }
                                            }
                                        }
                                        .into()
                                    })
                                    .collect();

                                rsx! {
                                    div { class: "mt-2",
                                        div { class: "flex gap-2 mb-2",
                                            button {
                                                class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                onclick: move |_| {
                                                    let ids: HashSet<String> = progs
                                                        .iter()
                                                        .filter(|p| {
                                                            filter.is_empty()
                                                                || p.name.to_lowercase().contains(&filter)
                                                        })
                                                        .map(|p| p.id.clone())
                                                        .collect();
                                                    selected_program_ids.set(ids);
                                                },
                                                { t!("specialist_programs_all_filtered") }
                                            }
                                            button {
                                                class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                onclick: move |_| selected_program_ids.set(HashSet::new()),
                                                { t!("specialist_programs_none") }
                                            }
                                        }
                                        div { class: "max-h-48 overflow-y-auto border border-border rounded-md p-1",
                                            {rows.into_iter()}
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "mb-6",
                            h3 { class: "text-lg font-semibold mb-2", { t!("specialist_programs_eligible_patients") } }
                            if selected_program_ids().is_empty() {
                                p { class: "text-sm text-text-muted", { t!("specialist_programs_select_programs_first") } }
                            } else {
                                label { class: "text-sm font-medium text-text", { t!("filter_patients_label") } }
                                input {
                                    class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface mb-4 focus:outline-none focus:border-primary",
                                    r#type: "text",
                                    placeholder:  t!("specialist_programs_filter_patients_by_name_or_email"),
                                    value: "{patient_filter()}",
                                    oninput: move |ev| patient_filter.set(ev.value().clone()),
                                }
                                    {
                                        let selected_prog_ids = selected_program_ids();
                                        let filter_pat = patient_filter().to_lowercase();
                                        let existing: HashSet<(String, String)> = assigns
                                            .iter()
                                            .filter_map(|a| {
                                                let pid = a.patient_id.clone();
                                                Some((pid, a.program_id.clone()))
                                            })
                                            .filter(|(_, prog_id)| selected_prog_ids.contains(prog_id))
                                            .collect();

                                        let rows: Vec<dioxus::prelude::Element> = links
                                            .iter()
                                            .filter_map(|link| {
                                                let profile = profiles.iter().find(|p| p.patient_id == link.patient_id)?;
                                                let has_any = selected_prog_ids
                                                    .iter()
                                                    .any(|prog_id| existing.contains(&(link.patient_id.clone(), prog_id.clone())));
                                                if has_any {
                                                    return None;
                                                }

                                                let label = format!("{} ({})", profile.full_name, profile.email);
                                                if !filter_pat.is_empty()
                                                    && !profile.full_name.to_lowercase().contains(&filter_pat)
                                                    && !profile.email.to_lowercase().contains(&filter_pat)
                                                {
                                                    return None;
                                                }

                                                let pid = link.patient_id.clone();
                                                let mut sel_pat = selected_patient_ids;
                                            let is_checked = sel_pat().contains(&pid);
                                            Some(rsx! {
                                                label { class: "flex items-center gap-2 p-2 min-h-11 cursor-pointer rounded hover:bg-gray-50",
                                                    input { class: "w-5 h-5",
                                                        r#type: "checkbox",
                                                        checked: is_checked,
                                                        onchange: move |ev| {
                                                            let mut set = sel_pat().clone();
                                                            if ev.checked() {
                                                                set.insert(pid.clone());
                                                            } else {
                                                                set.remove(&pid);
                                                            }
                                                            sel_pat.set(set);
                                                        },
                                                    }
                                                    span { "{label}" }
                                                }
                                            }
                                            .into())
                                        })
                                        .collect();

                                    rsx! {
                                        div { class: "mt-2",
                                            div { class: "flex gap-2 mb-2",
                                                button {
                                                    class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                    onclick: move |_| {
                                                        let selected_prog_ids = selected_program_ids();
                                                        let filter_pat = patient_filter().to_lowercase();
                                                        let existing: HashSet<(String, String)> = assigns
                                                            .iter()
                                                            .filter_map(|a| {
                                                                let pid = a.patient_id.clone();
                                                                Some((pid, a.program_id.clone()))
                                                            })
                                                            .filter(|(_, prog_id)| selected_prog_ids.contains(prog_id))
                                                            .collect();
                                                        let ids: HashSet<String> = links
                                                            .iter()
                                                            .filter_map(|link| {
                                                                let profile = profiles.iter().find(|p| p.patient_id == link.patient_id)?;
                                                                let has_any = selected_prog_ids
                                                                    .iter()
                                                                    .any(|prog_id| existing.contains(&(link.patient_id.clone(), prog_id.clone())));
                                                                if has_any {
                                                                    return None;
                                                                }
                                                                if !filter_pat.is_empty()
                                                                    && !profile.full_name.to_lowercase().contains(&filter_pat)
                                                                    && !profile.email.to_lowercase().contains(&filter_pat)
                                                                {
                                                                    return None;
                                                                }
                                                                Some(link.patient_id.clone())
                                                            })
                                                            .collect();
                                                        selected_patient_ids.set(ids);
                                                    },
                                                    { t!("specialist_programs_all_filtered") }
                                                }
                                                button {
                                                    class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                    onclick: move |_| selected_patient_ids.set(HashSet::new()),
                                                    { t!("specialist_programs_none") }
                                                }
                                            }
                                            div { class: "checkbox-list",
                                                {rows.into_iter()}
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex flex-wrap items-center gap-3 mt-4",
                            button {
                                class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                disabled: assign_program.state.read().is_loading() || selected_program_ids().is_empty() || selected_patient_ids().is_empty(),
                                onclick: move |_| {
                                    let prog_ids: Vec<String> = selected_program_ids().into_iter().collect();
                                    let patient_ids: Vec<String> = selected_patient_ids().into_iter().collect();
                                    let mut action = assign_program.action.clone();
                                    let mut selected_program_ids_ref = selected_program_ids;
                                    let mut selected_patient_ids_ref = selected_patient_ids;
                                    let mut resource = data.resource.clone();
                                    async move {
                                        action.call((patient_ids, prog_ids)).await;
                                        selected_program_ids_ref.set(HashSet::new());
                                        selected_patient_ids_ref.set(HashSet::new());
                                        resource.restart();
                                    }
                                },
                                { t!("assign_programs") }
                            }
                            button {
                                class: "min-h-11 px-4 font-medium rounded-md bg-secondary text-text hover:bg-secondary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                disabled: selected_program_ids().is_empty() && selected_patient_ids().is_empty(),
                                onclick: move |_| {
                                    selected_program_ids.set(HashSet::new());
                                    selected_patient_ids.set(HashSet::new());
                                },
                                { t!("clear_selection") }
                            }
                        }

                        if let Some(e) = assign_program.state.read().error() {
                            p { class: "text-error text-sm mt-2", "{e}" }
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
