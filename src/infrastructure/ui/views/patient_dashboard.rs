//! Patient dashboard: list active program(s) and agenda.
//! Al hacer clic en un día de entrenamiento se navega a una página específica
//! donde se muestran los ejercicios y se edita el feedback.

use dioxus::prelude::*;
use dioxus_router::{use_navigator, Link};

use crate::Route;

use crate::domain::entities::{
    ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutSession,
};
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::ui::components::AgendaBlock;

#[derive(Clone, PartialEq)]
struct PatientProgramData {
    patient_program_id: String,
    program_id: String,
    program_name: String,
    program_description: Option<String>,
    schedule: Vec<ProgramScheduleItem>,
    workouts: Vec<Workout>,
    sessions: Vec<WorkoutSession>,
    program_feedback: Vec<SessionExerciseFeedback>,
}

#[component]
pub fn PatientDashboard() -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();

    let data = use_resource(move || {
        let backend = backend.clone();
        let session = session_signal.read().clone();
        let backend = backend.clone();
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };
            let token = sess.access_token();
            let assignments = backend.list_active_patient_programs(token).await?;
            let mut out = Vec::new();
            for ass in assignments {
                let prog = match backend.get_program(token, &ass.program_id).await? {
                    Some(p) => p,
                    None => continue,
                };
                let workouts = backend
                    .list_workouts_for_program(token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                let schedule = backend
                    .list_program_schedule(token, &ass.program_id)
                    .await
                    .unwrap_or_default();
                let sessions = backend
                    .list_workout_sessions(token, &ass.id)
                    .await
                    .unwrap_or_default();
                let program_feedback = backend
                    .list_session_exercise_feedback_for_program(token, &ass.id)
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
                    program_feedback,
                });
            }
            Ok::<_, String>(out)
        }
    });

    let mut selected_for_feedback = use_signal(|| Option::<(String, i32)>::None);
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

    let programs = data
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned())
        .unwrap_or_default();

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::LoginView {}, "Ir a login" } }
        };
    }

    rsx! {
        div {
            class: "view container mx-auto patient-dashboard flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                h1 { class: "text-2xl font-semibold mb-4", "Mis programas de entrenamiento" }
                nav { class: "flex flex-wrap gap-2 mb-6 pb-4 border-b border-border",
                    Link { to: Route::LoginView {}, class: "text-primary no-underline text-sm min-h-11 inline-flex items-center px-2 rounded-md hover:bg-gray-100 hover:text-primary-hover", "Cerrar sesión" }
                }
                if programs.is_empty() && data.read().as_ref().as_ref().map(|r| r.is_ok()).unwrap_or(false) {
                    p { class: "text-text-muted italic", "No tienes programas activos asignados." }
                } else if data.read().as_ref().as_ref().map(|r| r.is_err()).unwrap_or(false) {
                    p { class: "text-error", "Error al cargar los programas." }
                } else if programs.is_empty() {
                    p { "Cargando..." }
                } else {
                    for prog in programs.iter() {
                        section { key: "{prog.patient_program_id}", class: "bg-surface border border-border rounded-md p-4 mb-4",
                            h2 { class: "text-xl font-semibold mt-0 mb-2", "{prog.program_name}" }
                            if let Some(ref desc) = prog.program_description {
                                p { "{desc}" }
                            }
                            AgendaBlock {
                                sessions: prog.sessions.clone(),
                                program_feedback: prog.program_feedback.clone(),
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
}
