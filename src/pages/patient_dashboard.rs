//! Patient dashboard: list active program(s), agenda by order (workout/rest days), select a day to add or edit feedback.
//! Session date defaults to today and can be changed.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::Route;

use crate::services::data::{
    build_agenda_schedule, complete_session, get_or_create_session, get_program,
    list_active_patient_programs, list_program_schedule, list_workout_sessions,
    list_workouts_for_program,
    uncomplete_session, update_session_feedback, ProgramScheduleItem, Workout, WorkoutSession,
};
use crate::components::AgendaBlock;
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
        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
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
                let workouts = list_workouts_for_program(&cfg, &sess.access_token, &ass.program_id).await.unwrap_or_default();
                let schedule = list_program_schedule(&cfg, &sess.access_token, &ass.program_id).await.unwrap_or_default();
                let sessions = list_workout_sessions(&cfg, &sess.access_token, &ass.id).await.unwrap_or_default();
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
    let mut effort = use_signal(|| 5i32);
    let mut pain = use_signal(|| 0i32);
    let mut comment = use_signal(|| String::new());
    let mut session_date = use_signal(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let mut submit_loading = use_signal(|| false);
    let mut submit_error = use_signal(|| Option::<String>::None);

    let programs = data.read().as_ref().and_then(|r| r.as_ref().ok().cloned()).unwrap_or_default();

    let selected_ctx = selected_for_feedback().and_then(|(pid, day_index)| {
        programs.iter().find(|p| p.patient_program_id == pid).map(|p| {
            let day_schedule = build_agenda_schedule(&p.schedule, &p.workouts);
            let type_label = day_schedule.iter().find(|(i, _, _)| *i == day_index).map(|(_, _, l)| l.clone()).unwrap_or_else(|| "—".to_string());
            let session = p.sessions.iter().find(|s| s.day_index == day_index).cloned();
            (p.clone(), day_index, type_label, session)
        })
    });

    use_effect(move || {
        let (pid, day_index) = match selected_for_feedback() {
            Some((a, b)) => (a, b),
            None => return,
        };
        let progs = data.read().as_ref().and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default();
        if let Some(prog) = progs.iter().find(|p| p.patient_program_id == pid) {
            if let Some(sess) = prog.sessions.iter().find(|s| s.day_index == day_index) {
                if sess.completed_at.is_some() {
                    effort.set(sess.effort.unwrap_or(5));
                    pain.set(sess.pain.unwrap_or(0));
                    comment.set(sess.comment.clone().unwrap_or_default());
                    session_date.set(sess.session_date.clone());
                } else {
                    session_date.set(chrono::Utc::now().format("%Y-%m-%d").to_string());
                }
            } else {
                session_date.set(chrono::Utc::now().format("%Y-%m-%d").to_string());
            }
        }
    });

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let (show_feedback, feedback_pid, feedback_day_index, feedback_label, feedback_completed, feedback_sid) =
        match &selected_ctx {
            Some((p, idx, label, sess)) => (
                true,
                p.patient_program_id.clone(),
                *idx,
                label.clone(),
                sess.as_ref().map(|s| s.completed_at.is_some()).unwrap_or(false),
                sess.as_ref().map(|s| s.id.clone()),
            ),
            None => (false, String::new(), 0, String::new(), false, None),
        };
    let feedback_sid_submit = feedback_sid.clone();
    let feedback_sid_uncomplete = feedback_sid.clone();

    rsx! {
        div { class: "patient-dashboard",
            h1 { "Mi programa de entrenamiento" }
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
                        h2 { "Programa: {prog.program_name}" }
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
                if show_feedback {
                    section { class: "feedback-section",
                        h2 { "Feedback — Día {feedback_day_index + 1}: {feedback_label}" }
                        div { class: "form",
                            label { "Fecha (cuando lo realizaste)"
                                input {
                                    r#type: "date",
                                    value: "{session_date()}",
                                    oninput: move |ev| session_date.set(ev.value().clone()),
                                }
                            }
                            label { "Esfuerzo (1-10)"
                                input {
                                    r#type: "number",
                                    min: "1",
                                    max: "10",
                                    value: "{effort()}",
                                    oninput: move |ev| effort.set(ev.value().parse().unwrap_or(5)),
                                }
                            }
                            label { "Dolor (0-10)"
                                input {
                                    r#type: "number",
                                    min: "0",
                                    max: "10",
                                    value: "{pain()}",
                                    oninput: move |ev| pain.set(ev.value().parse().unwrap_or(0)),
                                }
                            }
                            label { "Comentario (opcional)"
                                textarea {
                                    placeholder: "Comentario libre",
                                    value: "{comment()}",
                                    oninput: move |ev| comment.set(ev.value().clone()),
                                }
                            }
                            button {
                                disabled: submit_loading(),
                                onclick: move |_| {
                                    let pid = feedback_pid.clone();
                                    let day_idx = feedback_day_index;
                                    let date_str = session_date().clone();
                                    let is_compl = feedback_completed;
                                    let sid_opt = feedback_sid_submit.clone();
                                    let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                    let sess = session_signal.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let e = effort();
                                    let pa = pain();
                                    let c = comment().clone();
                                    submit_loading.set(true);
                                    submit_error.set(None);
                                    let mut refresh = data;
                                    spawn(async move {
                                        let res = if let Some(ref sid) = sid_opt {
                                            if is_compl {
                                                update_session_feedback(&cfg, &token, sid, Some(e), Some(pa), if c.is_empty() { None } else { Some(c.as_str()) }, Some(&date_str)).await
                                            } else {
                                                complete_session(&cfg, &token, sid, Some(e), Some(pa), if c.is_empty() { None } else { Some(c.as_str()) }).await
                                            }
                                        } else {
                                            match get_or_create_session(&cfg, &token, &pid, day_idx, &date_str).await {
                                                Ok(new_session) => complete_session(&cfg, &token, &new_session.id, Some(e), Some(pa), if c.is_empty() { None } else { Some(c.as_str()) }).await,
                                                Err(err) => {
                                                    submit_error.set(Some(err));
                                                    return;
                                                }
                                            }
                                        };
                                        match res {
                                            Ok(_) => {
                                                if !is_compl && sid_opt.is_none() { comment.set(String::new()); }
                                                refresh.restart();
                                            }
                                            Err(err) => submit_error.set(Some(err)),
                                        }
                                        submit_loading.set(false);
                                    });
                                },
                                if feedback_completed {
                                    "Guardar cambios"
                                } else {
                                    "Marcar completada y enviar feedback"
                                }
                            }
                            if feedback_completed {
                                button {
                                    class: "link-button",
                                    style: "margin-left: 0.5rem;",
                                    disabled: submit_loading(),
                                    onclick: move |_| {
                                        let Some(ref session_id) = feedback_sid_uncomplete else { return };
                                        let config = config_signal.read().clone().or_else(SupabaseConfig::from_env);
                                        let sess = session_signal.read().clone();
                                        let (cfg, token) = match (config, sess) {
                                            (Some(c), Some(s)) => (c, s.access_token),
                                            _ => return,
                                        };
                                        submit_loading.set(true);
                                        submit_error.set(None);
                                        let mut refresh = data;
                                        let session_id = session_id.clone();
                                        spawn(async move {
                                            let res = uncomplete_session(&cfg, &token, &session_id).await;
                                            match res {
                                                Ok(_) => refresh.restart(),
                                                Err(e) => submit_error.set(Some(e)),
                                            }
                                            submit_loading.set(false);
                                        });
                                    },
                                    "Marcar como no completado"
                                }
                            }
                        }
                        if let Some(ref e) = *submit_error.read() {
                            p { class: "error", "{e}" }
                        }
                        button {
                            class: "link-button",
                            onclick: move |_| selected_for_feedback.set(None),
                            "Cerrar"
                        }
                    }
                }
            }
        }
    }
}
