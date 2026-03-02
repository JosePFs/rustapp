//! Specialist view: progress of a patient's program(s).

const EMPTY: &str = "";

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::components::AgendaBlock;
use crate::Route;
use crate::services::data::{
    get_profiles_by_ids, get_program, list_patient_programs_for_specialist, list_program_schedule,
    list_workout_sessions, list_workouts_for_program, PatientProgram, Program, ProgramScheduleItem,
    WorkoutSession,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[derive(Clone, Debug)]
struct ProgramWithSessions {
    program: Program,
    assignment: PatientProgram,
    sessions: Vec<WorkoutSession>,
    schedule: Vec<ProgramScheduleItem>,
    workouts: Vec<crate::services::data::Workout>,
}

#[component]
pub fn PatientProgress(id: String) -> Element {
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let patient_id = id.clone();
    let pid = patient_id.clone();

    let data = use_resource(move || {
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        let pid2 = pid.clone();
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };
            let token = &sess.access_token;

            let profiles = get_profiles_by_ids(&cfg, token, &[pid2.clone()]).await?;
            let profile = profiles.into_iter().next().ok_or("Paciente no encontrado")?;

            let all_assignments = list_patient_programs_for_specialist(&cfg, token).await?;
            let assignments: Vec<&PatientProgram> = all_assignments
                .iter()
                .filter(|a| a.patient_id == pid2)
                .collect();

            let mut programs_with_sessions = Vec::new();
            for ass in assignments {
                let program = match get_program(&cfg, token, &ass.program_id).await? {
                    Some(p) => p,
                    None => continue,
                };
                let sessions = list_workout_sessions(&cfg, token, &ass.id).await.unwrap_or_default();
                let workouts = list_workouts_for_program(&cfg, token, &ass.program_id).await.unwrap_or_default();
                let schedule = list_program_schedule(&cfg, token, &ass.program_id).await.unwrap_or_default();
                programs_with_sessions.push(ProgramWithSessions {
                    program,
                    assignment: ass.clone(),
                    sessions,
                    schedule,
                    workouts,
                });
            }

            Ok::<_, String>((profile, programs_with_sessions))
        }
    });

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { class: "auth-required",
                p { "Debes iniciar sesión." }
                Link { to: Route::Login {}, "Ir a login" }
            }
        };
    }

    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
    if config.is_none() {
        return rsx! { p { "Configura SUPABASE_URL y SUPABASE_ANON_KEY." } };
    }

    rsx! {
        div { class: "patient-progress",
            nav { class: "nav",
                Link { to: Route::SpecialistDashboard {}, "← Panel del especialista" }
            }
            if let Some(Ok((ref profile, ref programs_with_sessions))) = data.read().as_ref() {
                h1 { "Progreso de {profile.full_name}" }
                p { class: "meta", "{profile.email}" }
                if programs_with_sessions.is_empty() {
                    p { class: "empty", "Este paciente no tiene ningún programa asignado." }
                } else {
                    for pws in programs_with_sessions.iter() {
                        section { class: "section program-progress",
                            h2 { "{pws.program.name}" }
                            if let Some(ref desc) = pws.program.description {
                                p { class: "description", "{desc}" }
                            }
                            p { class: "status", "Estado: {pws.assignment.status}" }
                            AgendaBlock {
                                sessions: pws.sessions.clone(),
                                schedule: pws.schedule.clone(),
                                workouts: pws.workouts.clone(),
                                title: "Agenda".to_string(),
                                patient_program_id: None,
                                write_selected_for_feedback: None,
                            }
                            if pws.sessions.is_empty() {
                                p { class: "empty", "Aún no hay sesiones registradas." }
                            } else {
                                table { class: "sessions-table",
                                    thead {
                                        tr {
                                            th { "Día" }
                                            th { "Fecha" }
                                            th { "Completada" }
                                            th { "Esfuerzo" }
                                            th { "Dolor" }
                                            th { "Comentario" }
                                        }
                                    }
                                    tbody {
                                        for s in pws.sessions.iter() {
                                            tr { key: "{s.id}",
                                                td { "Día {s.day_index + 1}" }
                                                td { "{s.session_date}" }
                                                td { if s.completed_at.is_some() { "Sí" } else { "No" } }
                                                td { "{s.effort.as_ref().map(|e| e.to_string()).unwrap_or_default()}" }
                                                td { "{s.pain.as_ref().map(|p| p.to_string()).unwrap_or_default()}" }
                                                td { "{s.comment.as_deref().unwrap_or(EMPTY)}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if data.read().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                p { class: "error", "Error al cargar el progreso del paciente." }
            } else {
                p { "Cargando..." }
            }
        }
    }
}
