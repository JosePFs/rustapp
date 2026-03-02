//! Página de detalle de un día de entrenamiento para el paciente:
//! muestra los ejercicios del entrenamiento y permite guardar/editar el feedback.

use dioxus::prelude::*;
use dioxus_router::Link;

use crate::services::data::{
    build_agenda_schedule, complete_session, get_or_create_session, get_patient_program_by_id,
    get_program, list_exercises_for_workout, list_program_schedule, list_workout_sessions,
    list_workouts_for_program, uncomplete_session, update_session_feedback, Exercise,
    ProgramScheduleItem, Workout, WorkoutSession,
};
use crate::services::supabase_client::{AuthSession, SupabaseConfig};
use crate::Route;

#[derive(Clone)]
struct WorkoutDayDetail {
    patient_program_id: String,
    program_name: String,
    workout_name: String,
    day_index: i32,
    session: Option<WorkoutSession>,
    exercises: Vec<Exercise>,
}

#[component]
pub fn PatientWorkoutDay(patient_program_id: String, day_index: String) -> Element {
    let config_signal = use_context::<Signal<Option<SupabaseConfig>>>();
    let session_signal = use_context::<Signal<Option<AuthSession>>>();
    let day_index_val = day_index.parse::<i32>().unwrap_or(0);

    let data = use_resource(move || {
        let config = config_signal
            .read()
            .clone()
            .or_else(SupabaseConfig::from_env);
        let session = session_signal.read().clone();
        let pid = patient_program_id.clone();
        let day_idx = day_index_val;
        async move {
            let (cfg, sess) = match (config, session) {
                (Some(c), Some(s)) => (c, s),
                _ => return Err("No config or session".to_string()),
            };

            let pp_opt = get_patient_program_by_id(&cfg, &sess.access_token, &pid).await?;
            let pp = match pp_opt {
                Some(p) if p.status == "active" => p,
                _ => return Err("No se encuentra la asignación activa".to_string()),
            };

            let program = match get_program(&cfg, &sess.access_token, &pp.program_id).await? {
                Some(p) => p,
                None => return Err("Programa no encontrado".to_string()),
            };

            let schedule: Vec<ProgramScheduleItem> =
                list_program_schedule(&cfg, &sess.access_token, &pp.program_id)
                    .await
                    .unwrap_or_default();
            let workouts: Vec<Workout> =
                list_workouts_for_program(&cfg, &sess.access_token, &pp.program_id)
                    .await
                    .unwrap_or_default();
            let sessions: Vec<WorkoutSession> =
                list_workout_sessions(&cfg, &sess.access_token, &pp.id)
                    .await
                    .unwrap_or_default();

            let day_schedule = build_agenda_schedule(&schedule, &workouts);
            let (_, workout_id_opt, label) = day_schedule
                .iter()
                .find(|(i, _, _)| *i == day_idx)
                .cloned()
                .ok_or_else(|| "Día no encontrado en la programación".to_string())?;

            let workout_id = match workout_id_opt {
                Some(id) => id,
                None => return Err("Este día es de descanso (sin entrenamiento)".to_string()),
            };

            let workout_name = workouts
                .iter()
                .find(|w| w.id == workout_id)
                .map(|w| w.name.clone())
                .unwrap_or(label);

            let exercises =
                list_exercises_for_workout(&cfg, &sess.access_token, &workout_id).await?;

            let session_for_day = sessions.into_iter().find(|s| s.day_index == day_idx);

            Ok::<_, String>(WorkoutDayDetail {
                patient_program_id: pp.id,
                program_name: program.name,
                workout_name,
                day_index: day_idx,
                session: session_for_day,
                exercises,
            })
        }
    });

    let mut effort = use_signal(|| 5i32);
    let mut pain = use_signal(|| 0i32);
    let mut comment = use_signal(|| String::new());
    let mut session_date = use_signal(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let mut submit_loading = use_signal(|| false);
    let mut submit_error = use_signal(|| Option::<String>::None);

    // Sincronizar el formulario cuando se cargan los datos o la sesión.
    {
        let data = data.clone();
        use_effect(move || {
            let detail = data.read().as_ref().and_then(|r| r.as_ref().ok().cloned());
            if let Some(d) = detail {
                if let Some(sess) = d.session {
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
    }

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::Login {}, "Ir a login" } }
        };
    }

    let detail = data.read().as_ref().and_then(|r| r.as_ref().ok().cloned());

    let (program_name, workout_name, day_idx, session_opt) = match &detail {
        Some(d) => (
            d.program_name.clone(),
            d.workout_name.clone(),
            d.day_index,
            d.session.clone(),
        ),
        None => (String::new(), String::new(), day_index_val, None),
    };

    let feedback_completed = session_opt
        .as_ref()
        .map(|s| s.completed_at.is_some())
        .unwrap_or(false);
    let feedback_sid = session_opt.as_ref().map(|s| s.id.clone());
    let feedback_sid_submit = feedback_sid.clone();
    let feedback_sid_uncomplete = feedback_sid.clone();

    let config_signal_clone = config_signal.clone();
    let session_signal_clone = session_signal.clone();
    let data_clone = data.clone();

    rsx! {
        div { class: "patient-workout-day",
            h1 { "Mi programa de entrenamiento" }
            nav { class: "nav",
                Link { to: Route::PatientDashboard {}, "← Volver a mi programa" }
            }
            if let Some(err) = data.read().as_ref().and_then(|r| r.as_ref().err()).cloned() {
                p { class: "error", "{err}" }
            } else if detail.is_none() {
                p { "Cargando..." }
            } else {
                h2 { "Entrenamiento: {workout_name}" }
                if !program_name.is_empty() {
                    p { class: "muted", "Programa: {program_name}" }
                }

                section { class: "section",
                    h3 { "Ejercicios" }
                    if let Some(d) = &detail {
                        if d.exercises.is_empty() {
                            p { class: "muted", "Este entrenamiento no tiene ejercicios configurados." }
                        } else {
                            ul {
                                for ex in d.exercises.iter() {
                                    li {
                                        key: "{ex.id}",
                                        strong { "{ex.name}" }
                                        if let Some(desc) = &ex.description {
                                            if !desc.is_empty() {
                                                span { " — {desc}" }
                                            }
                                        }
                                        if let Some(url) = &ex.video_url {
                                            if !url.is_empty() {
                                                span { " " }
                                                a { href: "{url}", target: "_blank", rel: "noopener", "Vídeo" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "section",
                    h3 { "Feedback del día" }
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
                                let date_str = session_date().clone();
                                let config = config_signal_clone
                                    .read()
                                    .clone()
                                    .or_else(SupabaseConfig::from_env);
                                let sess = session_signal_clone.read().clone();
                                let (cfg, token) = match (config, sess) {
                                    (Some(c), Some(s)) => (c, s.access_token),
                                    _ => return,
                                };
                                let e = effort();
                                let pa = pain();
                                let c = comment().clone();
                                let pid = detail
                                    .as_ref()
                                    .map(|d| d.patient_program_id.clone())
                                    .unwrap_or_default();
                                let di = day_idx;
                                let sid_opt = feedback_sid_submit.clone();
                                let completed = feedback_completed;
                                submit_loading.set(true);
                                submit_error.set(None);
                                let mut data = data_clone.clone();
                                spawn(async move {
                                    let res = if let Some(ref sid) = sid_opt {
                                        if completed {
                                            update_session_feedback(
                                                &cfg,
                                                &token,
                                                sid,
                                                Some(e),
                                                Some(pa),
                                                if c.is_empty() { None } else { Some(c.as_str()) },
                                                Some(&date_str),
                                            )
                                            .await
                                        } else {
                                            complete_session(
                                                &cfg,
                                                &token,
                                                sid,
                                                Some(e),
                                                Some(pa),
                                                if c.is_empty() { None } else { Some(c.as_str()) },
                                            )
                                            .await
                                        }
                                    } else {
                                        match get_or_create_session(
                                            &cfg,
                                            &token,
                                            &pid,
                                            di,
                                            &date_str,
                                        )
                                        .await
                                        {
                                            Ok(new_session) => {
                                                complete_session(
                                                    &cfg,
                                                    &token,
                                                    &new_session.id,
                                                    Some(e),
                                                    Some(pa),
                                                    if c.is_empty() {
                                                        None
                                                    } else {
                                                        Some(c.as_str())
                                                    },
                                                )
                                                .await
                                            }
                                            Err(err) => {
                                                submit_error.set(Some(err));
                                                return;
                                            }
                                        }
                                    };
                                    match res {
                                        Ok(_) => {
                                            if !completed && sid_opt.is_none() {
                                                comment.set(String::new());
                                            }
                                            data.restart();
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
                                    let config = config_signal_clone
                                        .read()
                                        .clone()
                                        .or_else(SupabaseConfig::from_env);
                                    let sess = session_signal_clone.read().clone();
                                    let (cfg, token) = match (config, sess) {
                                        (Some(c), Some(s)) => (c, s.access_token),
                                        _ => return,
                                    };
                                    let session_id = session_id.clone();
                                    let mut data = data.clone();
                                    submit_loading.set(true);
                                    submit_error.set(None);
                                    spawn(async move {
                                        let res = uncomplete_session(&cfg, &token, &session_id).await;
                                        match res {
                                            Ok(_) => data.restart(),
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
                }
            }
        }
    }
}
