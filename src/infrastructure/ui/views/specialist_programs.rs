use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoInformationCircle;
use dioxus_free_icons::Icon;
use dioxus_primitives::ContentSide;
use dioxus_router::Link;

use crate::domain::error::DomainError;
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::ui::components::{Tooltip, TooltipContent, TooltipTrigger};
use crate::Route;

#[component]
pub fn SpecialistPrograms() -> Element {
    let app_context = use_context::<AppContext>();
    let session_signal = app_context.session();
    let backend = app_context.backend();

    // Pacientes del especialista (para mostrar nombres/emails en la asignación)
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

    // Programas del especialista
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

    // Asignaciones programas-pacientes
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

    // Estado local para crear programas
    let mut new_program_name = use_signal(|| String::new());
    let mut new_program_desc = use_signal(|| String::new());
    let mut create_program_error = use_signal(|| Option::<String>::None);
    let mut create_program_loading = use_signal(|| false);

    // Estado local para asignación de programas
    let mut program_filter = use_signal(|| String::new());
    let mut patient_filter = use_signal(|| String::new());
    let mut selected_program_ids = use_signal(|| HashSet::<String>::new());
    let mut selected_patient_ids = use_signal(|| HashSet::<String>::new());
    let mut assign_error = use_signal(|| Option::<String>::None);
    let mut assign_loading = use_signal(|| false);

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

    // Datos precalculados para la asignación.
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

    let backend_create_program = backend.clone();
    let backend_assign = backend.clone();

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
                                span { "Programas" }
                                span { class: "text-xs", if nav_open() { "▲" } else { "▼" } }
                            }
                            if nav_open() {
                                div { class: "absolute z-10 mt-2 w-56 bg-surface border border-border rounded-md shadow-md flex flex-col py-1",
                                    Link { to: Route::SpecialistPatients {}, class: "px-3 py-2 text-sm text-primary no-underline hover:bg-gray-100 hover:text-primary-hover", "Pacientes" }
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
                        h2 { class: "text-xl font-semibold m-0", "Programas" }
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
                                h4 { style: "margin-top: 0; margin-bottom: 8px;", "Programas" }
                                p { style: "margin: 0; margin-bottom: 4px;", "Crea programas y edítalos." }
                                p { style: "margin: 0;", "Después podrás asignarlos a tus pacientes." }
                            }
                        }
                    }
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
                                                "Todos (filtrados)"
                                            }
                                            button {
                                                class: "bg-transparent text-primary underline min-h-0 py-1 text-sm",
                                                onclick: move |_| selected_program_ids.set(HashSet::new()),
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
                                    let existing: HashSet<(String, String)> = assigns
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
                                                        let existing: HashSet<(String, String)> = assigns
                                                            .iter()
                                                            .filter(|a| selected_prog_ids.contains(&a.program_id))
                                                            .map(|a| (a.patient_id.clone(), a.program_id.clone()))
                                                            .collect();
                                                        let ids: HashSet<String> = links
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
                                                    onclick: move |_| selected_patient_ids.set(HashSet::new()),
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

                        // Botones de asignar y limpiar selección
                        div { class: "flex flex-wrap items-center gap-3 mt-4",
                            button {
                                class: "min-h-11 px-4 font-medium rounded-md bg-primary text-white hover:bg-primary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                disabled: assign_loading() || selected_program_ids().is_empty() || selected_patient_ids().is_empty(),
                                onclick: move |_| {
                                    let backend = backend_assign.clone();
                                    let session = session_signal.read().clone();
                                    let Some(sess) = session else { return };
                                    let token = sess.access_token().to_string();
                                    let prog_ids: Vec<String> = selected_program_ids().into_iter().collect();
                                    let patient_ids: Vec<String> = selected_patient_ids().into_iter().collect();

                                    assign_loading.set(true);
                                    assign_error.set(None);

                                    let mut assignments_ref = assignments;
                                    let mut selected_program_ids_ref = selected_program_ids;
                                    let mut selected_patient_ids_ref = selected_patient_ids;

                                    spawn(async move {
                                        let mut any_error = None;
                                        for pid in patient_ids.iter() {
                                            for prog_id in prog_ids.iter() {
                                                if let Err(e) = backend.assign_program_to_patient(&token, prog_id, pid).await {
                                                    any_error = Some(e);
                                                    break;
                                                }
                                            }
                                            if any_error.is_some() {
                                                break;
                                            }
                                        }

                                        if let Some(e) = any_error {
                                            assign_error.set(Some(e));
                                        } else {
                                            // Limpiar selección y refrescar asignaciones
                                            selected_program_ids_ref.set(HashSet::new());
                                            selected_patient_ids_ref.set(HashSet::new());
                                            assignments_ref.restart();
                                        }

                                        assign_loading.set(false);
                                    });
                                },
                                "Asignar programas"
                            }
                            button {
                                class: "min-h-11 px-4 font-medium rounded-md bg-secondary text-text hover:bg-secondary-hover disabled:opacity-60 disabled:cursor-not-allowed",
                                disabled: selected_program_ids().is_empty() && selected_patient_ids().is_empty(),
                                onclick: move |_| {
                                    selected_program_ids.set(HashSet::new());
                                    selected_patient_ids.set(HashSet::new());
                                },
                                "Limpiar selección"
                            }
                        }

                        if let Some(ref e) = *assign_error.read() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }

                        // Nota: la lista de asignaciones actuales se ha simplificado para evitar
                        // complejidades de préstamos; aquí solo mostramos el formulario de asignación.
                    } else {
                        p { "Cargando datos de programas y pacientes..." }
                    }
                }
            }
        }
    }
}
