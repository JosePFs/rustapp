//! Specialist view: progress of a patient's program(s).

const EMPTY: &str = "";

fn session_avg_feedback(
    session_id: &str,
    feedback: &[SessionExerciseFeedback],
) -> (String, String) {
    let sess_fb: Vec<_> = feedback
        .iter()
        .filter(|f| f.workout_session_id == session_id)
        .collect();
    if sess_fb.is_empty() {
        (EMPTY.to_string(), EMPTY.to_string())
    } else {
        let e: f64 =
            sess_fb.iter().filter_map(|f| f.effort).sum::<i32>() as f64 / sess_fb.len() as f64;
        let p: f64 =
            sess_fb.iter().filter_map(|f| f.pain).sum::<i32>() as f64 / sess_fb.len() as f64;
        (format!("{:.1}", e), format!("{:.1}", p))
    }
}

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::domain::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, Workout,
    WorkoutSession,
};
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::ui::components::AgendaBlock;
use crate::Route;

#[derive(Clone, Debug)]
struct ProgramWithSessions {
    program: Program,
    assignment: PatientProgram,
    sessions: Vec<WorkoutSession>,
    program_feedback: Vec<SessionExerciseFeedback>,
    schedule: Vec<ProgramScheduleItem>,
    workouts: Vec<Workout>,
}

#[component]
pub fn PatientProgress(id: String) -> Element {
    let app_context = use_context::<AppContext>();
    let session_signal = app_context.session();
    let backend = app_context.backend();
    let patient_id = id.clone();
    let pid = patient_id.clone();

    let data = use_resource(move || {
        let session = session_signal.read().clone();
        let pid2 = pid.clone();
        let backend = backend.clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let token = sess.access_token();

            let profiles = backend
                .get_profiles_by_ids(&[pid2.clone()], token)
                .await
                .map_err(|e| e.to_string())?;
            let profile = profiles
                .into_iter()
                .next()
                .ok_or("Paciente no encontrado")?;

            let all_assignments = backend.list_patient_programs_for_specialist(token).await?;
            let assignments: Vec<&PatientProgram> = all_assignments
                .iter()
                .filter(|a| a.patient_id == pid2)
                .collect();

            let mut programs_with_sessions = Vec::new();
            for ass in assignments {
                let program = match backend.get_program(token, &ass.program_id).await? {
                    Some(p) => p,
                    None => continue,
                };
                let sessions = backend
                    .list_workout_sessions(token, &ass.id)
                    .await
                    .unwrap_or_default();
                let program_feedback = backend
                    .list_session_exercise_feedback_for_program(token, &ass.id)
                    .await
                    .unwrap_or_default();
                let workouts = backend
                    .list_workouts_for_program(token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                let schedule = backend
                    .list_program_schedule(token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                programs_with_sessions.push(ProgramWithSessions {
                    program,
                    assignment: ass.clone(),
                    sessions,
                    program_feedback,
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
            div {
                class: "p-6 text-center",
                p { "Debes iniciar sesión." }
                Link { to: Route::LoginView {}, class: "text-primary underline", "Ir a login" }
            }
        };
    }

    rsx! {
        div {
            class: "view container mx-auto patient-progress flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                nav { class: "flex flex-wrap gap-2 mb-6 pb-4 border-b border-border",
                    Link { to: Route::SpecialistDashboard {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100 hover:text-primary-hover", "← Panel del especialista" }
                }
                if let Some(Ok((ref profile, ref programs_with_sessions))) = data.read().as_ref() {
                    h1 { class: "text-2xl font-semibold mb-4", "Progreso de {profile.full_name()}" }
                    p { class: "text-sm text-text-muted mb-4", "{profile.email()}" }
                    if programs_with_sessions.is_empty() {
                        p { class: "text-text-muted italic py-4", "Este paciente no tiene ningún programa asignado." }
                    } else {
                        for pws in programs_with_sessions.iter() {
                            section {
                                class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                                h2 { class: "text-xl font-semibold mt-0 mb-2", "{pws.program.name}" }
                                if let Some(ref desc) = pws.program.description {
                                    p { class: "text-sm text-text-muted mb-1", "{desc}" }
                                }
                                p { class: "text-xs text-text-muted mb-2", "Estado: {pws.assignment.status}" }
                                AgendaBlock {
                                    sessions: pws.sessions.clone(),
                                    program_feedback: pws.program_feedback.clone(),
                                    schedule: pws.schedule.clone(),
                                    workouts: pws.workouts.clone(),
                                    title: "Agenda".to_string(),
                                    patient_program_id: None,
                                    write_selected_for_feedback: None,
                                }
                                if pws.sessions.is_empty() {
                                    p { class: "text-text-muted italic py-4", "Aún no hay sesiones registradas." }
                                } else {
                                    div { class: "overflow-x-auto",
                                        table { class: "border-collapse text-sm w-full whitespace-nowrap",
                                            thead {
                                                tr {
                                                    th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", "Día" }
                                                    th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", "Fecha" }
                                                    th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", "Completada" }
                                                    th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", "Esf. medio" }
                                                    th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", "Dolor medio" }
                                                }
                                            }
                                            tbody {
                                                for s in pws.sessions.iter() {
                                                    tr { key: "{s.id}", class: "border-b border-border",
                                                        td { class: "p-2", "Día {s.day_index + 1}" }
                                                        td { class: "p-2", "{s.session_date}" }
                                                        td { class: "p-2", if s.completed_at.is_some() { "Sí" } else { "No" } }
                                                        td { class: "p-2", "{session_avg_feedback(&s.id, &pws.program_feedback).0}" }
                                                        td { class: "p-2", "{session_avg_feedback(&s.id, &pws.program_feedback).1}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if data.read().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                    p { class: "text-error", "Error al cargar el progreso del paciente." }
                } else {
                    p { "Cargando..." }
                }
            }
        }
    }
}
