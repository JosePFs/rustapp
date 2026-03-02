//! Specialist dashboard: list patients, add patient, programs, assign, compliance.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;
use crate::services::data::{
    assign_program_to_patient, create_program, get_profiles_by_ids,
    list_patient_programs_for_specialist, list_programs, list_specialist_patients,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[component]
fn PatientAssignRow(
    patient_id: String,
    name: String,
    is_selected: bool,
    on_toggle: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "checkbox-row",
            input {
                r#type: "checkbox",
                checked: is_selected,
                onchange: move |ev| on_toggle.call(ev.checked()),
            }
            span { "{name}" }
        }
    }
}

#[component]
fn PatientAssignList(
    list: Vec<(crate::services::data::SpecialistPatient, crate::services::data::Profile)>,
    selected_patient_ids: Signal<std::collections::HashSet<String>>,
) -> Element {
    let rows: Vec<_> = list
        .iter()
        .map(|tuple| {
            let pid = tuple.0.patient_id.clone();
            let name = format!("{} ({})", tuple.1.full_name, tuple.1.email);
            let mut sid = selected_patient_ids;
            rsx! {
                PatientAssignRow {
                    patient_id: pid.clone(),
                    name: name.clone(),
                    is_selected: sid().contains(&pid),
                    on_toggle: move |checked| {
                        let mut set = sid().clone();
                        if checked {
                            set.insert(pid.clone());
                        } else {
                            set.remove(&pid);
                        }
                        sid.set(set);
                    },
                }
            }
            .into()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();
    rsx! {
        {rows.into_iter()}
    }
}

#[component]
pub fn SpecialistDashboard() -> Element {
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();

    let patients = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let links = list_specialist_patients(&cfg, &sess.access_token).await?;
            let ids: Vec<String> = links.iter().map(|l| l.patient_id.clone()).collect();
            let profiles = get_profiles_by_ids(&cfg, &sess.access_token, &ids).await?;
            Ok::<_, String>((links, profiles))
        }
    });

    let programs = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            list_programs(&cfg, &sess.access_token).await
        }
    });

    let assignments = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            list_patient_programs_for_specialist(&cfg, &sess.access_token).await
        }
    });

    let mut new_program_name = use_signal(|| String::new());
    let mut new_program_desc = use_signal(|| String::new());
    let mut create_program_error = use_signal(|| Option::<String>::None);
    let mut create_program_loading = use_signal(|| false);

    let mut patient_filter = use_signal(|| String::new());
    let mut selected_patient_ids = use_signal(|| std::collections::HashSet::<String>::new());
    let mut assign_program_id = use_signal(|| Option::<String>::None);
    let mut assign_error = use_signal(|| Option::<String>::None);
    let mut assign_loading = use_signal(|| false);

    let filtered_patients = use_memo(move || {
        let (links, profiles) = match patients.read().as_ref() {
            Some(Ok((l, p))) => (l.clone(), p.clone()),
            _ => return vec![],
        };
        let f = patient_filter().to_lowercase();
        links
            .iter()
            .filter_map(|l| {
                let profile = profiles.iter().find(|p| p.id == l.patient_id)?.clone();
                let link = l.clone();
                let show = f.is_empty()
                    || profile.full_name.to_lowercase().contains(&f)
                    || profile.email.to_lowercase().contains(&f);
                if show {
                    Some((link, profile))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    });

    let session = session_signal.read().clone();
    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);

    if session.is_none() {
        return rsx! {
            div { class: "auth-required",
                p { "Debes iniciar sesión." }
                Link { to: Route::Login {}, "Ir a login" }
            }
        };
    }

    let _sess = session.as_ref().unwrap();

    if config.is_none() {
        return rsx! {
            p { "Configura SUPABASE_URL y SUPABASE_ANON_KEY." }
        };
    }

    let assign_list = filtered_patients.read().clone();

    rsx! {
        div { class: "specialist-dashboard",
            h1 { "Panel del especialista" }
            nav { class: "nav",
                Link { to: Route::ExerciseLibrary {}, "Biblioteca de ejercicios" }
                Link { to: Route::WorkoutLibrary {}, "Biblioteca de entrenamientos" }
                Link { to: Route::Login {}, "Cerrar sesión" }
            }

            section { class: "section",
                h2 { "Pacientes" }
                p { class: "hint", "Haz clic en un paciente para ver su progreso. Asigna programas en la sección inferior." }
                if let Some(Ok((links, profiles))) = patients.read().as_ref() {
                    ul { class: "patient-list",
                        for link in links.iter() {
                            li {
                                key: "{link.id}",
                                Link {
                                    to: Route::PatientProgress { id: link.patient_id.clone() },
                                    class: "patient-link",
                                    { profiles.iter().find(|p| p.id == link.patient_id).map(|p| rsx! { "{p.full_name} ({p.email})" }).unwrap_or(rsx! { "{link.patient_id}" }) }
                                }
                            }
                        }
                    }
                } else if patients.read().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                    p { class: "error", "Error al cargar pacientes" }
                } else {
                    p { "Cargando pacientes..." }
                }
            }

            section { class: "section",
                h2 { "Programas" }
                if let Some(Ok(progs)) = programs.read().as_ref() {
                    ul {
                        for p in progs.iter() {
                            li {
                                key: "{p.id}",
                                Link { to: Route::ProgramEditor { id: p.id.clone() }, "{p.name}" }
                            }
                        }
                    }
                } else {
                    p { "Cargando programas..." }
                }
                div { class: "form new-program",
                    input {
                        placeholder: "Nombre del programa",
                        value: "{new_program_name()}",
                        oninput: move |ev| new_program_name.set(ev.value().clone()),
                    }
                    input {
                        placeholder: "Descripción (opcional)",
                        value: "{new_program_desc()}",
                        oninput: move |ev| new_program_desc.set(ev.value().clone()),
                    }
                    button {
                        disabled: create_program_loading(),
                        onclick: move |_| {
                            let name = new_program_name().clone();
                            if name.is_empty() { return; }
                            let desc = new_program_desc().clone();
                            let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                            let session = session_signal.read().clone();
                            let (cfg, token, specialist_id) = match (config, session) {
                                (Some(c), Some(s)) => (c, s.access_token, s.user.id.clone()),
                                _ => return,
                            };
                            create_program_loading.set(true);
                            create_program_error.set(None);
                            let mut refresh = programs;
                            spawn(async move {
                                match create_program(&cfg, &token, &specialist_id, &name, if desc.is_empty() { None } else { Some(&desc) }).await {
                                    Ok(_) => { new_program_name.set(String::new()); new_program_desc.set(String::new()); refresh.restart(); }
                                    Err(e) => create_program_error.set(Some(e)),
                                }
                                create_program_loading.set(false);
                            });
                        },
                        "Crear programa"
                    }
                    if let Some(ref e) = *create_program_error.read() {
                        p { class: "error", "{e}" }
                    }
                }
            }

            section { class: "section",
                h2 { "Asignar programa a paciente" }
                if let Some(Ok(progs)) = programs.read().as_ref() {
                    p { class: "hint", "Selecciona uno o más pacientes y un programa, luego pulsa Asignar." }
                    input {
                        class: "filter-input",
                        r#type: "text",
                        placeholder: "Filtrar por nombre o email...",
                        value: "{patient_filter()}",
                        oninput: move |ev| patient_filter.set(ev.value().clone()),
                    }
                    div { class: "patient-multi-select",
                        "Seleccionar: "
                        button {
                            class: "link-button",
                            onclick: move |_| {
                                let ids: std::collections::HashSet<String> = filtered_patients
                                    .read()
                                    .iter()
                                    .map(|(l, _)| l.patient_id.clone())
                                    .collect();
                                selected_patient_ids.set(ids);
                            },
                            "Todos (filtrados)"
                        }
                        button {
                            class: "link-button",
                            onclick: move |_| selected_patient_ids.set(std::collections::HashSet::new()),
                            "Ninguno"
                        }
                        div { class: "patient-checkbox-list",
                            PatientAssignList {
                                list: assign_list.clone(),
                                selected_patient_ids,
                            }
                        }
                    }
                    div { class: "form assign-row",
                        label { "Programa"
                            select {
                                oninput: move |ev| assign_program_id.set(Some(ev.value().clone())),
                                option { value: "", "Seleccionar programa" }
                                for p in progs.iter() {
                                    option { value: "{p.id}", "{p.name}" }
                                }
                            }
                        }
                        button {
                            disabled: assign_loading() || selected_patient_ids().is_empty() || assign_program_id().is_none(),
                            onclick: move |_| {
                                let prog_id = match assign_program_id().clone() {
                                    Some(id) => id,
                                    None => return,
                                };
                                let ids: Vec<String> = selected_patient_ids().iter().cloned().collect();
                                if ids.is_empty() { return; }
                                let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                let session = session_signal.read().clone();
                                let (cfg, token) = match (config, session) {
                                    (Some(c), Some(s)) => (c, s.access_token),
                                    _ => return,
                                };
                                assign_loading.set(true);
                                assign_error.set(None);
                                let mut refresh = assignments;
                                spawn(async move {
                                    let mut err = None;
                                    for pid in ids {
                                        if let Err(e) = assign_program_to_patient(&cfg, &token, &pid, &prog_id).await {
                                            err = Some(e);
                                            break;
                                        }
                                    }
                                    if let Some(e) = err {
                                        assign_error.set(Some(e));
                                    } else {
                                        refresh.restart();
                                    }
                                    assign_loading.set(false);
                                });
                            },
                            "Asignar seleccionados"
                        }
                    }
                }
                if let Some(ref e) = *assign_error.read() {
                    p { class: "error", "{e}" }
                }
            }

            section { class: "section",
                h2 { "Cumplimiento (asignaciones activas)" }
                if let Some(Ok(assigns)) = assignments.read().as_ref() {
                    {
                        let programs_ok = programs.read().as_ref().and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default();
                        let profiles: Vec<crate::services::data::Profile> = patients
                            .read()
                            .as_ref()
                            .and_then(|r| r.as_ref().ok())
                            .map(|(_links, p)| p.clone())
                            .unwrap_or_default();
                        let active = assigns.iter().filter(|a| a.status == "active");
                        let rows: Vec<dioxus::prelude::Element> = active
                            .map(|a| {
                                let patient_label = profiles
                                    .iter()
                                    .find(|p| p.id == a.patient_id)
                                    .map(|p| format!("{} ({})", p.full_name, p.email))
                                    .unwrap_or_else(|| a.patient_id.clone());
                                let program_label = programs_ok
                                    .iter()
                                    .find(|pr| pr.id == a.program_id)
                                    .map(|pr| pr.name.clone())
                                    .unwrap_or_else(|| a.program_id.clone());
                                let pid = a.patient_id.clone();
                                rsx! {
                                    li { key: "{a.id}",
                                        Link { to: Route::PatientProgress { id: pid.clone() }, "{patient_label}" }
                                        span { " — " }
                                        span { "{program_label}" }
                                    }
                                }
                                .into()
                            })
                            .collect();
                        rsx! {
                            ul {
                                {rows.into_iter()}
                            }
                        }
                    }
                } else {
                    p { "Cargando..." }
                }
            }
        }
    }
}
