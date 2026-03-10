//! Specialist patients view: list patients and add existing patients.

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoInformationCircle;
use dioxus_free_icons::Icon;
use dioxus_primitives::ContentSide;
use dioxus_router::Link;

use crate::domain::error::DomainError;
use crate::domain::profile::Profile;
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::ui::components::{Tooltip, TooltipContent, TooltipTrigger};
use crate::Route;

#[component]
pub fn SpecialistPatients() -> Element {
    let app_context = use_context::<AppContext>();
    let session_signal = app_context.session();
    let backend = app_context.backend();

    // Pacientes del especialista
    let backend_patients = backend.clone();
    let patients = use_resource(move || {
        let session = session_signal.read().clone();
        let backend = backend_patients.clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let links = backend
                .list_specialist_patients(sess.access_token())
                .await?;
            let ids: Vec<String> = links.iter().map(|l| l.patient_id.clone()).collect();
            let profiles = backend
                .get_profiles_by_ids(&ids, sess.access_token())
                .await
                .map_err(|e| DomainError::Api(e.to_string()));
            Ok::<_, String>((links, profiles))
        }
    });

    // Estado local para añadir paciente existente
    let mut add_patient_email = use_signal(|| String::new());
    let mut add_patient_loading = use_signal(|| false);
    let mut add_patient_error = use_signal(|| Option::<String>::None);

    let session = session_signal.read().clone();

    if session.is_none() {
        return rsx! {
            div { class: "p-6 text-center",
                p { "Debes iniciar sesión." }
                Link { to: Route::LoginView {}, class: "text-primary underline", "Ir a login" }
            }
        };
    }

    let _sess = session.as_ref().unwrap();

    let backend_add_patient = backend.clone();

    rsx! {
        div {
            class: "view container mx-auto specialist-dashboard",
            div {
                class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                {
                    // Navbar desplegable: actúa como título de la página.
                    let mut nav_open = use_signal(|| false);
                    rsx! {
                        nav { class: "relative mb-6",
                            button {
                                class: "min-h-11 px-0 bg-transparent text-2xl font-semibold inline-flex items-center gap-2 text-text",
                                onclick: move |_| {
                                    nav_open.set(!nav_open());
                                },
                                span { "Pacientes" }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPrograms {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Programas y asignación" }
                                    Link { to: Route::ExerciseLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de ejercicios" }
                                    Link { to: Route::WorkoutLibrary {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de entrenamientos" }
                                    Link { to: Route::LoginView {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Cerrar sesión" }
                                }
                            }
                        }
                    }
                }

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    div { class: "flex items-center gap-2 mt-0 mb-2",
                        h2 { class: "text-xl font-semibold m-0", "Pacientes" }
                        Tooltip {
                            TooltipTrigger {
                                style: "vertical-align: bottom;",
                                Icon {
                                    width: 24,
                                    height: 24,
                                    icon: IoInformationCircle,
                                }
                            }
                            TooltipContent { side: ContentSide::Bottom, style: "width: 300px;",
                                h4 { style: "margin-top: 0; margin-bottom: 8px;", "Pacientes y progreso" }
                                p { style: "margin: 0; margin-bottom: 4px;", "Haz clic en un paciente para ver su progreso." }
                                p { style: "margin: 0;", "Puedes asignar programas desde la sección de Programas y asignación." }
                            }
                        }
                    }
                    if let Some(Ok((links, profiles))) = patients.read().as_ref() {
                        ul { class: "list-none p-0 m-0 mb-4",
                            for link in links.iter() {
                                li { key: "{link.id}", class: "mb-1",
                                    Link {
                                        to: Route::PatientProgress { id: link.patient_id.clone() },
                                        class: "block p-4 min-h-11 text-primary no-underline rounded-md border border-border bg-surface hover:bg-gray-50 hover:border-primary",
                                        {
                                            profiles
                                                .as_ref()
                                                .unwrap()
                                                .iter()
                                                .find(|p| p.id().value() == link.patient_id)
                                                .map(|p: &Profile| rsx! { "{p.full_name()} ({p.email()})" })
                                                .unwrap_or(rsx! { "{link.patient_id}" })
                                        }
                                    }
                                }
                            }
                        }
                        // Añadir paciente existente (por email) a este especialista.
                        div { class: "flex flex-col gap-4 mt-4",
                            h3 { class: "text-lg font-semibold mb-0", "Añadir paciente existente" }
                            p { class: "text-sm text-text-muted mb-0", "Introduce el email de un paciente para vincularlo a ti." }
                            div { class: "flex flex-wrap gap-2 items-center",
                                input {
                                    class: "flex-1 min-w-40 min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                                    placeholder: "Email del paciente",
                                    value: "{add_patient_email()}",
                                    oninput: move |ev| add_patient_email.set(ev.value().clone()),
                                }
                                button {
                                    class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                    disabled: add_patient_loading() || add_patient_email().trim().is_empty(),
                                    onclick: move |_| {
                                        let email_val = add_patient_email().trim().to_string();
                                        if email_val.is_empty() { return; }
                                        let backend = backend_add_patient.clone();
                                        let session = session_signal.read().clone();
                                        let Some(sess) = session else { return };
                                        let token = sess.access_token().to_string();
                                        let specialist_id = sess.user_id().to_string();
                                        add_patient_loading.set(true);
                                        add_patient_error.set(None);
                                        let mut patients_ref = patients;
                                        spawn(async move {
                                            match backend.get_patient_id_by_email(&token, &email_val).await {
                                                Ok(Some(patient_id)) => {
                                                    match backend.add_specialist_patient(&token, &specialist_id, &patient_id).await {
                                                        Ok(_) => {
                                                            add_patient_email.set(String::new());
                                                            patients_ref.restart();
                                                        }
                                                        Err(e) => add_patient_error.set(Some(e)),
                                                    }
                                                }
                                                Ok(None) => add_patient_error.set(Some("No se encontró un paciente con ese email".to_string())),
                                                Err(e) => add_patient_error.set(Some(e)),
                                            }
                                            add_patient_loading.set(false);
                                        });
                                    },
                                    "Vincular paciente"
                                }
                            }
                            if let Some(ref e) = *add_patient_error.read() {
                                p { class: "text-error text-sm mt-2", "{e}" }
                            }
                        }
                    } else if patients.read().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                        p { class: "text-error", "Error al cargar pacientes" }
                    } else {
                        p { "Cargando pacientes..." }
                    }
                }
            }
        }
    }
}
