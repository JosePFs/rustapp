//! Specialist dashboard: list patients, add patient, programs, assign, compliance.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::services::data::{
    add_specialist_patient, assign_program_to_patient, create_program, get_patient_id_by_email,
    get_profiles_by_ids, list_patient_programs_for_specialist, list_programs,
    list_specialist_patients, unassign_program_from_patient,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};
use crate::Route;

#[component]
pub fn SpecialistDashboard() -> Element {
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();

    let patients = use_resource(move || {
        let config = config_signal
            .read()
            .clone()
            .or_else(SupabaseConfig::from_env);
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
        let config = config_signal
            .read()
            .clone()
            .or_else(SupabaseConfig::from_env);
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
        let config = config_signal
            .read()
            .clone()
            .or_else(SupabaseConfig::from_env);
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

    let mut program_filter = use_signal(|| String::new());
    let mut patient_filter = use_signal(|| String::new());
    let mut selected_program_ids = use_signal(|| std::collections::HashSet::<String>::new());
    let mut selected_patient_ids = use_signal(|| std::collections::HashSet::<String>::new());
    let mut assign_error = use_signal(|| Option::<String>::None);
    let mut assign_loading = use_signal(|| false);

    let session = session_signal.read().clone();
    let config = config_signal
        .read()
        .clone()
        .or_else(SupabaseConfig::from_env);

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

    // Form state para añadir paciente por email
    let mut add_patient_email = use_signal(|| String::new());
    let mut add_patient_loading = use_signal(|| false);
    let mut add_patient_error = use_signal(|| Option::<String>::None);

    // Datos precalculados para la asignación (para evitar préstamos de corta vida en los handlers).
    let assign_data = match (
        programs.read().as_ref(),
        assignments.read().as_ref(),
        patients.read().as_ref(),
    ) {
        (Some(Ok(progs)), Some(Ok(assigns)), Some(Ok((links, profiles)))) => Some((
            progs.clone(),
            assigns.clone(),
            links.clone(),
            profiles.clone(),
        )),
        _ => None,
    };

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
                    // Añadir paciente existente (por email) a este especialista.
                    div { class: "form add-patient",
                        h3 { "Añadir paciente existente" }
                        p { class: "hint", "Introduce el email de un paciente para vincularlo a ti." }
                        div { class: "row",
                            input {
                                placeholder: "Email del paciente",
                                value: "{add_patient_email()}",
                                oninput: move |ev| add_patient_email.set(ev.value().clone()),
                            }
                            button {
                                disabled: add_patient_loading() || add_patient_email().trim().is_empty(),
                                onclick: move |_| {
                                    let email_val = add_patient_email().trim().to_string();
                                    if email_val.is_empty() { return; }
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let session = session_signal.read().clone();
                                    let (cfg, sess) = match (config, session) {
                                        (Some(c), Some(s)) => (c, s),
                                        _ => return,
                                    };
                                    let token = sess.access_token.clone();
                                    let specialist_id = sess.user.id.clone();
                                    add_patient_loading.set(true);
                                    add_patient_error.set(None);
                                    let mut patients_ref = patients;
                                    spawn(async move {
                                        match get_patient_id_by_email(&cfg, &token, &email_val).await {
                                            Ok(Some(patient_id)) => {
                                                match add_specialist_patient(&cfg, &token, &specialist_id, &patient_id).await {
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
                            p { class: "error", "{e}" }
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
                h2 { "Asignar programas a pacientes" }
                if let Some((progs, assigns, links, profiles)) = assign_data.clone() {
                    p { class: "hint",
                        "1) Selecciona uno o varios programas. 2) Selecciona pacientes que aún no tengan ninguno de esos programas. 3) Pulsa Asignar."
                    }

                    // Bloque: selector múltiple de programas con filtro
                    div { class: "assign-programs-block",
                        h3 { "Programas" }
                        input {
                            class: "filter-input",
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
                                        label { class: "checkbox-row",
                                            input {
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
                                div { class: "multi-select",
                                    div { class: "multi-select-actions",
                                        button {
                                            class: "link-button",
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
                                            class: "link-button",
                                            onclick: move |_| selected_program_ids.set(std::collections::HashSet::new()),
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

                    // Bloque: selector múltiple de pacientes elegibles con filtro
                    div { class: "assign-patients-block",
                        h3 { "Pacientes elegibles" }
                        if selected_program_ids().is_empty() {
                            p { class: "muted", "Selecciona primero uno o más programas para ver los pacientes disponibles." }
                        } else {
                            input {
                                class: "filter-input",
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
                                        let profile = profiles.iter().find(|p| p.id == link.patient_id)?.clone();

                                        // Elegible si NO tiene ninguno de los programas seleccionados
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
                                            label { class: "checkbox-row",
                                                input {
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
                                    div { class: "multi-select",
                                        div { class: "multi-select-actions",
                                            button {
                                                class: "link-button",
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
                                                            let profile = profiles.iter().find(|p| p.id == link.patient_id)?;
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
                    div { class: "form assign-row",
                        button {
                            disabled: assign_loading() || selected_program_ids().is_empty() || selected_patient_ids().is_empty(),
                            onclick: move |_| {
                                let prog_ids: Vec<String> = selected_program_ids().iter().cloned().collect();
                                let pat_ids: Vec<String> = selected_patient_ids().iter().cloned().collect();
                                if prog_ids.is_empty() || pat_ids.is_empty() { return; }
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
                                    let mut err: Option<String> = None;
                                    'outer: for prog_id in prog_ids.iter() {
                                        for pid in pat_ids.iter() {
                                            if let Err(e) = assign_program_to_patient(&cfg, &token, pid, prog_id).await {
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
                                let assign_id = a.id.clone();
                                let mut assigns_res = assignments;
                                rsx! {
                                    li { key: "{assign_id}",
                                        Link { to: Route::PatientProgress { id: pid.clone() }, "{patient_label}" }
                                        span { " — " }
                                        span { "{program_label}" }
                                        button {
                                            class: "btn-small danger",
                                            style: "margin-left: 0.5rem;",
                                            onclick: move |_| {
                                                let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                                let session = session_signal.read().clone();
                                                let (cfg, token) = match (config, session) {
                                                    (Some(c), Some(s)) => (c, s.access_token),
                                                    _ => return,
                                                };
                                                let id = assign_id.clone();
                                                spawn(async move {
                                                    let _ = unassign_program_from_patient(&cfg, &token, &id).await;
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
