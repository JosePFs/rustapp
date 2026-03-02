//! Patient dashboard: list active program(s) and agenda.
//! Al hacer clic en un día de entrenamiento se navega a una página específica
//! donde se muestran los ejercicios y se edita el feedback.

use dioxus::prelude::*;
use dioxus_router::{use_navigator, Link};

use crate::Route;

use crate::components::AgendaBlock;
use crate::services::data::{
    get_program, list_active_patient_programs, list_program_schedule, list_workout_sessions,
    list_workouts_for_program, ProgramScheduleItem, Workout, WorkoutSession,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};

#[derive(Clone, PartialEq)]
struct PatientProgramData {
    patient_program_id: String,
    program_id: String,
    program_name: String,
    program_description: Option<String>,
    schedule: Vec<ProgramScheduleItem>,
    workouts: Vec<Workout>,
    sessions: Vec<WorkoutSession>,
}

#[component]
pub fn PatientDashboard() -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();

    let data = use_resource(move || {
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
            let assignments = list_active_patient_programs(&cfg, &sess.access_token).await?;
            let mut out = Vec::new();
            for ass in assignments {
                let prog = match get_program(&cfg, &sess.access_token, &ass.program_id).await? {
                    Some(p) => p,
                    None => continue,
                };
                let workouts = list_workouts_for_program(&cfg, &sess.access_token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                let schedule = list_program_schedule(&cfg, &sess.access_token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                let sessions = list_workout_sessions(&cfg, &sess.access_token, &ass.id)
                    .await
                    .unwrap_or_default();
                out.push(PatientProgramData {
                    patient_program_id: ass.id.clone(),
                    program_id: ass.program_id.clone(),
                    program_name: prog.name,
                    program_description: prog.description,
                    schedule,
                    workouts,
                    sessions,
                });
            }
            Ok::<_, String>(out)
        }
    });

    let mut selected_for_feedback = use_signal(|| Option::<(String, i32)>::None);
    let programs = data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let navigator = use_navigator();

    // Cuando el paciente selecciona un día de entrenamiento en la agenda,
    // navegamos a la página de detalle de ese día (entrenamiento + feedback).
    use_effect(move || {
        let (pid, day_index) = match selected_for_feedback() {
            Some(v) => v,
            None => return,
        };
        navigator.push(Route::PatientWorkoutDay {
            patient_program_id: pid,
            day_index: day_index.to_string(),
        });
        selected_for_feedback.set(None);
    });

    rsx! {
        div { class: "patient-dashboard",
            h1 { "Mis programas de entrenamiento" }
            nav { class: "nav",
                Link { to: Route::Login {}, "Cerrar sesión" }
            }
            if programs.is_empty() && data.read().as_ref().as_ref().map(|r| r.is_ok()).unwrap_or(false) {
                p { "No tienes programas activos asignados." }
            } else if data.read().as_ref().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                p { "Error al cargar los programas." }
            } else if programs.is_empty() {
                p { "Cargando..." }
            } else {
                for prog in programs.iter() {
                    section { key: "{prog.patient_program_id}", class: "program-block",
                        h2 { "{prog.program_name}" }
                        if let Some(ref desc) = prog.program_description {
                            p { "{desc}" }
                        }
                        AgendaBlock {
                            sessions: prog.sessions.clone(),
                            schedule: prog.schedule.clone(),
                            workouts: prog.workouts.clone(),
                            title: "Agenda".to_string(),
                            patient_program_id: Some(prog.patient_program_id.clone()),
                            write_selected_for_feedback: Some(selected_for_feedback),
                        }
                    }
                }
            }
        }
    }
}
