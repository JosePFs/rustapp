//! Specialist dashboard: list patients, add patient, programs, assign, compliance.

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
pub fn SpecialistDashboard() -> Element {
    let app_context = use_context::<AppContext>();
    let session_signal = app_context.session();
    let backend = app_context.backend();

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

    let backend_programs = backend.clone();
    let programs = use_resource(move || {
        let session = session_signal.read().clone();
        let backend = backend_programs.clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            backend.list_programs(sess.access_token()).await
        }
    });

    let backend_assignments = backend.clone();
    let assignments = use_resource(move || {
        let session = session_signal.read().clone();
        let backend = backend_assignments.clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            backend
                .list_patient_programs_for_specialist(sess.access_token())
                .await
        }
    });

    let mut new_program_name = use_signal(|| String::new());
    let mut new_program_desc = use_signal(|| String::new());
    let mut create_program_error = use_signal(|| Option::<String>::None);
    let mut create_program_loading = use_signal(|| false);

    let mut program_filter = use_signal(|| String::new());
    let mut patient_filter = use_signal(|| String::new());
    let mut selected_program_ids = use_signal(|| std::collections::HashSet::<String>::new());
    let mut selected_patient_ids = use_signal(|| std::collections::HashSet::<String>::new());
    let mut assign_error = use_signal(|| Option::<String>::None);
    let mut assign_loading = use_signal(|| false);

    let mut add_patient_email = use_signal(|| String::new());
    let mut add_patient_loading = use_signal(|| false);
    let mut add_patient_error = use_signal(|| Option::<String>::None);

    let session = session_signal.read().clone();

    if session.is_none() {
        return rsx! {
            div { class: "p-6 text-center",
                p { "Debes iniciar sesión." }
                Link { to: Route::Login {}, class: "text-primary underline", "Ir a login" }
            }
        };
    }

    let _sess = session.as_ref().unwrap();

    // Datos precalculados para la asignación (para evitar préstamos de corta vida en los handlers).
    let programs_ref = programs.read();
    let assignments_ref = assignments.read();
    let patients_ref = patients.read();
    let assign_data = match (
        programs_ref.as_ref(),
        assignments_ref.as_ref(),
        patients_ref.as_ref(),
    ) {
        (Some(Ok(progs)), Some(Ok(assigns)), Some(Ok((links, profiles)))) => Some((
            progs.clone(),
            assigns.clone(),
            links.clone(),
            profiles.as_ref().ok().cloned().unwrap_or_default(),
        )),
        _ => None,
    };

    let backend_add_patient = backend.clone();
    let backend_create_program = backend.clone();
    let backend_assign = backend.clone();
    let backend_for_unassign = backend.clone();

    rsx! {
        div {
            class: "view container mx-auto specialist-dashboard flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                h1 { class: "text-2xl font-semibold mb-4", "Panel del especialista" }
                nav { class: "flex flex-wrap gap-2 mb-6 pb-4 border-b border-border",
                    Link { to: Route::ExerciseLibrary {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de ejercicios" }
                    Link { to: Route::WorkoutLibrary {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100 hover:text-primary-hover", "Biblioteca de entrenamientos" }
                    Link { to: Route::Login {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100 hover:text-primary-hover", "Cerrar sesión" }
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
                                p { style: "margin: 0;", "Asigna programas en la sección inferior." }
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
                                        { profiles.as_ref().unwrap().iter().find(|p| p.id().value() == link.patient_id).map(|p| rsx! { "{p.full_name()} ({p.email()})" }).unwrap_or(rsx! { "{link.patient_id}" }) }
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

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", "Programas" }
                    if let Some(Ok(progs)) = programs.read().as_ref() {
                        ul { class: "list-none p-0 m-0 mb-4",
                            for p in progs.iter() {
                                li { key: "{p.id}", class: "mb-1",
                                    Link { to: Route::ProgramEditor { id: p.id.clone() }, class: "text-primary no-underline hover:underline", "{p.name}" }
                                }
                            }
                        }
                    } else {
                        p { "Cargando programas..." }
                    }
                    div { class: "flex flex-col gap-4 mt-4",
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                            placeholder: "Nombre del programa",
                            value: "{new_program_name()}",
                            oninput: move |ev| new_program_name.set(ev.value().clone()),
                        }
                        input {
                            class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                            placeholder: "Descripción (opcional)",
                            value: "{new_program_desc()}",
                            oninput: move |ev| new_program_desc.set(ev.value().clone()),
                        }
                        button {
                            class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60",
                            disabled: create_program_loading(),
                            onclick: move |_| {
                                let name = new_program_name().clone();
                                if name.is_empty() { return; }
                                let desc = new_program_desc().clone();
                                let backend = backend_create_program.clone();
                                let session = session_signal.read().clone();
                                let Some(sess) = session else { return };
                                let token = sess.access_token().to_string();
                                let specialist_id = sess.user_id().to_string();
                                create_program_loading.set(true);
                                create_program_error.set(None);
                                let mut refresh = programs;
                                spawn(async move {
                                    match backend.create_program(&token, &specialist_id, &name, if desc.is_empty() { None } else { Some(&desc) }).await {
                                        Ok(_) => { new_program_name.set(String::new()); new_program_desc.set(String::new()); refresh.restart(); }
                                        Err(e) => create_program_error.set(Some(e)),
                                    }
                                    create_program_loading.set(false);
                                });
                            },
                            "Crear programa"
                        }
                        if let Some(ref e) = *create_program_error.read() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", "Asignar programas a pacientes" }
                    if let Some((progs, assigns, links, profiles)) = assign_data.clone() {
                        p { class: "text-sm text-text-muted mb-4",
                            "1) Selecciona uno o varios programas. 2) Selecciona pacientes que aún no tengan ninguno de esos programas. 3) Pulsa Asignar."
                        }

                        // Bloque: selector múltiple de programas con filtro
                        div { class: "mb-6",
                            h3 { class: "text-lg font-semibold mb-2", "Programas" }
                            input {
                                class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface mb-4 focus:outline-none focus:border-primary",
                                r#type: "text",
                                placeholder: "Filtrar programas por nombre...",
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
                                                    let ids: std::collections::HashSet<String> = progs
                                                        .iter()
                                                        .filter(|p| {
                                                            filter.is_empty()
                                                                || p.name.to_lowercase().contains(&filter)
                                                        })
                                                        .map(|p| p.id.clone())
                                                        .collect();
                                                    selected_program_ids.set(ids);
                                                },
                                                "Todos (filtrados)"
                                            }
                                            button {
                                                class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                onclick: move |_| selected_program_ids.set(std::collections::HashSet::new()),
                                                "Ninguno"
                                            }
                                        }
                                        div { class: "max-h-48 overflow-y-auto border border-border rounded-md p-1",
                                            {rows.into_iter()}
                                        }
                                    }
                                }
                            }
                        }

                        // Bloque: selector múltiple de pacientes elegibles con filtro
                        div { class: "mb-6",
                            h3 { class: "text-lg font-semibold mb-2", "Pacientes elegibles" }
                            if selected_program_ids().is_empty() {
                                p { class: "text-sm text-text-muted", "Selecciona primero uno o más programas para ver los pacientes disponibles." }
                            } else {
                                input {
                                    class: "w-full min-h-11 px-4 border border-border rounded-md bg-surface mb-4 focus:outline-none focus:border-primary",
                                    r#type: "text",
                                    placeholder: "Filtrar pacientes por nombre o email...",
                                    value: "{patient_filter()}",
                                    oninput: move |ev| patient_filter.set(ev.value().clone()),
                                }
                                {
                                    // Prepara estructuras auxiliares
                                    let selected_prog_ids = selected_program_ids();
                                    let filter_pat = patient_filter().to_lowercase();
                                    let existing: std::collections::HashSet<(String, String)> = assigns
                                        .iter()
                                        .filter(|a| selected_prog_ids.contains(&a.program_id))
                                        .map(|a| (a.patient_id.clone(), a.program_id.clone()))
                                        .collect();

                                    let rows: Vec<dioxus::prelude::Element> = links
                                        .iter()
                                        .filter_map(|link| {
                                            let profile = profiles.iter().find(|p| p.id().value() == link.patient_id)?.clone();

                                            // Elegible si NO tiene ninguno de los programas seleccionados
                                            let has_any = selected_prog_ids
                                                .iter()
                                                .any(|prog_id| existing.contains(&(link.patient_id.clone(), prog_id.clone())));
                                            if has_any {
                                                return None;
                                            }

                                            let label = format!("{} ({})", profile.full_name(), profile.email());
                                            if !filter_pat.is_empty()
                                                && !profile.full_name().value().to_lowercase().contains(&filter_pat)
                                                && !profile.email().value().to_lowercase().contains(&filter_pat)
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
                                                        let existing: std::collections::HashSet<(String, String)> = assigns
                                                            .iter()
                                                            .filter(|a| selected_prog_ids.contains(&a.program_id))
                                                            .map(|a| (a.patient_id.clone(), a.program_id.clone()))
                                                            .collect();
                                                        let ids: std::collections::HashSet<String> = links
                                                            .iter()
                                                            .filter_map(|link| {
                                                                let profile = profiles.iter().find(|p| p.id().value() == link.patient_id)?;
                                                                let has_any = selected_prog_ids
                                                                    .iter()
                                                                    .any(|prog_id| existing.contains(&(link.patient_id.clone(), prog_id.clone())));
                                                                if has_any {
                                                                    return None;
                                                                }
                                                                if !filter_pat.is_empty()
                                                                    && !profile.full_name().value().to_lowercase().contains(&filter_pat)
                                                                    && !profile.email().value().to_lowercase().contains(&filter_pat)
                                                                {
                                                                    return None;
                                                                }
                                                                Some(link.patient_id.clone())
                                                            })
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
                                            }
                                            div { class: "checkbox-list",
                                                {rows.into_iter()}
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Botón de asignación batch: todos los programas seleccionados x todos los pacientes seleccionados
                        div { class: "mt-4",
                            button {
                                class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                disabled: assign_loading() || selected_program_ids().is_empty() || selected_patient_ids().is_empty(),
                                onclick: move |_| {
                                    let prog_ids: Vec<String> = selected_program_ids().iter().cloned().collect();
                                    let pat_ids: Vec<String> = selected_patient_ids().iter().cloned().collect();
                                    if prog_ids.is_empty() || pat_ids.is_empty() { return; }
                                    let backend = backend_assign.clone();
                                    let session = session_signal.read().clone();
                                    let Some(sess) = session else { return };
                                    let token = sess.access_token().to_string();
                                    assign_loading.set(true);
                                    assign_error.set(None);
                                    let mut refresh = assignments;
                                    spawn(async move {
                                        let mut err: Option<String> = None;
                                        'outer: for prog_id in prog_ids.iter() {
                                            for pid in pat_ids.iter() {
                                                if let Err(e) = backend.assign_program_to_patient(&token, pid, prog_id).await {
                                                    err = Some(e);
                                                    break 'outer;
                                                }
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
                                "Asignar programas a pacientes seleccionados"
                            }
                        }
                    } else {
                        p { "Cargando datos para asignación..." }
                    }
                    if let Some(ref e) = *assign_error.read() {
                        p { class: "text-error text-sm mt-2", "{e}" }
                    }
                }

                section { class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                    h2 { class: "text-xl font-semibold mt-0 mb-2", "Cumplimiento (asignaciones activas)" }
                    if let Some(Ok(assigns)) = assignments.read().as_ref() {
                        {
                            let programs_ok = programs.read().as_ref().and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default();
                            let profiles: Vec<Profile> = patients
                                .read()
                                .as_ref()
                                .and_then(|r| r.as_ref().ok())
                                .map(|(_links, p)| p.as_ref().ok().cloned().unwrap_or_default())
                                .unwrap_or_default();
                            let active = assigns.iter().filter(|a| a.status == "active");
                            let rows: Vec<dioxus::prelude::Element> = active
                                .map(|a| {
                                    let patient_label = profiles
                                        .iter()
                                        .find(|p| p.id().value() == a.patient_id)
                                        .map(|p| format!("{} ({})", p.full_name(), p.email()))
                                        .unwrap_or_else(|| a.patient_id.clone());
                                    let program_label = programs_ok
                                        .iter()
                                        .find(|pr| pr.id == a.program_id)
                                        .map(|pr| pr.name.clone())
                                        .unwrap_or_else(|| a.program_id.clone());
                                    let pid = a.patient_id.clone();
                                    let assign_id = a.id.clone();
                                    let mut assigns_res = assignments;
                                    let backend_unassign = backend_for_unassign.clone();
                                    rsx! {
                                        li { key: "{assign_id}", class: "flex flex-wrap items-center gap-1 py-2 border-b border-border last:border-0",
                                            Link { to: Route::PatientProgress { id: pid.clone() }, class: "text-primary no-underline hover:underline", "{patient_label}" }
                                            span { " — " }
                                            span { "{program_label}" }
                                            button {
                                                class: "min-h-9 px-2 text-sm rounded-md bg-error text-white hover:opacity-90 ml-2",
                                                onclick: move |_| {
                                                    let backend = backend_unassign.clone();
                                                    let session = session_signal.read().clone();
                                                    let Some(sess) = session else { return };
                                                    let token = sess.access_token().to_string();
                                                    let id = assign_id.clone();
                                                    spawn(async move {
                                                        let _ = backend.unassign_program_from_patient(&token, &id).await;
                                                        assigns_res.restart();
                                                    });
                                                },
                                                "Desasignar"
                                            }
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
}
