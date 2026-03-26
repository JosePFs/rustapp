use dioxus::prelude::*;

use dioxus_free_icons::icons::io_icons::IoAdd;
use dioxus_free_icons::Icon;
use dioxus_i18n::t;
use dioxus_router::Link;

use crate::hooks::{
    add_specialist_patient::use_add_specialist_patient,
    specialist_patients::use_specialist_patients, AsyncState,
};
use crate::Route;

#[component]
pub fn SpecialistPatients() -> Element {
    let patients = use_specialist_patients();
    let add_patient = use_add_specialist_patient();

    let mut add_patient_email = use_signal(|| String::new());

    use_effect(move || {
        if add_patient.state.read().is_ready() {
            add_patient_email.set(String::new());
        }
    });

    let resource = patients.resource.clone();

    rsx! {
        div {
            class: "view container mx-auto specialist-dashboard w-full",
            div { class: "content w-full",
                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    div { class: "flex gap-2 mt-0 mb-2",
                        p { class: "text-sm text-text-muted mb-4", "Haz clic en un paciente para ver su progreso. Puedes asignar programas desde la sección de Programas y asignación." }
                    }
                    {
                        match &*patients.state.read() {
                            AsyncState::Idle | AsyncState::Loading => rsx! {
                                p { "Cargando pacientes..." }
                            },
                            AsyncState::Error(_) => rsx! {
                                p { class: "text-error", { t!("error_load_patients") } }
                            },
                            AsyncState::Ready(data) => rsx! {
                                ul { class: "list-none p-0 m-0 mb-4",
                                    for link in data.links.iter() {
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
                                div { class: "flex flex-col gap-4 mt-4",
                                    h3 { class: "text-lg font-semibold mb-0", { t!("add_patient_existing") } }
                                    p { class: "text-sm text-text-muted mb-0", { t!("add_patient_hint") } }
                                    div { class: "flex flex-wrap gap-2 items-center",
                                        input {
                                            class: "flex-1 min-w-40 min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                                            placeholder: "{t!(\"add_patient_email_placeholder\")}",
                                            value: "{add_patient_email()}",
                                            oninput: move |ev| add_patient_email.set(ev.value().clone()),
                                        }
                                        button {
                                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed flex items-center gap-2",
                                            disabled: add_patient.state.read().is_loading() || add_patient_email().trim().is_empty(),
                                            onclick: move |_| {
                                                let email_val = add_patient_email().trim().to_string();
                                                if email_val.is_empty() { return; }
                                                let mut action = add_patient.action.clone();
                                                let mut resource_clone = resource.clone();
                                                spawn(async move {
                                                    action.call(email_val).await;
                                                    resource_clone.restart();
                                                });
                                            },
                                            Icon { width: 18, height: 18, icon: IoAdd }
                                            { t!("add_patient_link") }
                                        }
                                    }
                                    if let Some(e) = add_patient.state.read().error() {
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
