//! Página de detalle de un día de entrenamiento para el paciente:
//! muestra los ejercicios del entrenamiento y permite guardar/editar el feedback.

use dioxus::prelude::*;
use dioxus_primitives::slider::SliderValue;
use dioxus_router::Link;

use crate::domain::entities::{
    ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutExercise, WorkoutSession,
};
use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::supabase::api::build_agenda_schedule;
use crate::infrastructure::ui::components::{
    Backview, Slider, SliderRange, SliderThumb, SliderTrack, Textarea, TextareaVariant,
};
use crate::Route;

#[derive(Clone)]
struct WorkoutDayDetail {
    patient_program_id: String,
    program_name: String,
    workout_name: String,
    workout_description: Option<String>,
    day_index: i32,
    session: Option<WorkoutSession>,
    exercises: Vec<WorkoutExercise>,
    feedback: Vec<SessionExerciseFeedback>,
}

#[component]
pub fn PatientWorkoutDay(patient_program_id: String, day_index: String) -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();
    let day_index_val = day_index.parse::<i32>().unwrap_or(0);

    let backend_for_resource = backend.clone();
    let data = use_resource(move || {
        let backend = backend_for_resource.clone();
        let session = session_signal.read().clone();
        let pid = patient_program_id.clone();
        let day_idx = day_index_val;
        async move {
            let sess = match session {
                Some(s) => s,
                None => return Err("No session".to_string()),
            };

            let pp_opt = backend
                .get_patient_program_by_id(sess.access_token(), &pid)
                .await?;
            let pp = match pp_opt {
                Some(p) if p.status == "active" => p,
                _ => return Err("No se encuentra la asignación activa".to_string()),
            };

            let program = match backend
                .get_program(sess.access_token(), &pp.program_id)
                .await?
            {
                Some(p) => p,
                None => return Err("Programa no encontrado".to_string()),
            };

            let schedule: Vec<ProgramScheduleItem> = backend
                .list_program_schedule(sess.access_token(), &pp.program_id)
                .await
                .unwrap_or_default();
            let workouts: Vec<Workout> = backend
                .list_workouts_for_program(sess.access_token(), &pp.program_id)
                .await
                .unwrap_or_default();
            let sessions: Vec<WorkoutSession> = backend
                .list_workout_sessions(sess.access_token(), &pp.id)
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

            let (workout_name, workout_description) = workouts
                .iter()
                .find(|w| w.id == workout_id)
                .map(|w| (w.name.clone(), w.description.clone()))
                .unwrap_or((label, None));

            let exercises = backend
                .list_exercises_for_workout(sess.access_token(), &workout_id)
                .await?;

            let session_for_day = sessions.into_iter().find(|s| s.day_index == day_idx);
            let feedback = if let Some(ref s) = session_for_day {
                backend
                    .list_session_exercise_feedback(sess.access_token(), &s.id)
                    .await
                    .unwrap_or_default()
            } else {
                vec![]
            };

            Ok::<_, String>(WorkoutDayDetail {
                patient_program_id: pp.id,
                program_name: program.name,
                workout_name,
                workout_description,
                day_index: day_idx,
                session: session_for_day,
                exercises,
                feedback,
            })
        }
    });

    let mut session_date = use_signal(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let mut exercise_feedback =
        use_signal(|| std::collections::HashMap::<String, (i32, i32, String)>::new());
    let mut submit_loading = use_signal(|| false);
    let mut submit_error = use_signal(|| Option::<String>::None);

    {
        let data = data.clone();
        use_effect(move || {
            let detail = data.read().as_ref().and_then(|r| r.as_ref().ok().cloned());
            if let Some(d) = detail {
                if let Some(ref sess) = d.session {
                    session_date.set(sess.session_date.clone());
                } else {
                    session_date.set(chrono::Utc::now().format("%Y-%m-%d").to_string());
                }
                let mut map = std::collections::HashMap::new();
                for we in &d.exercises {
                    let (effort, pain, comment) = d
                        .feedback
                        .iter()
                        .find(|f| f.exercise_id == we.exercise.id)
                        .map(|f| {
                            (
                                f.effort.unwrap_or(5),
                                f.pain.unwrap_or(0),
                                f.comment.clone().unwrap_or_default(),
                            )
                        })
                        .unwrap_or((5, 0, String::new()));
                    map.insert(we.exercise.id.clone(), (effort, pain, comment));
                }
                exercise_feedback.set(map);
            }
        });
    }

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::LoginView {}, "Ir a login" } }
        };
    }

    let detail = data.read().as_ref().and_then(|r| r.as_ref().ok().cloned());

    let (program_name, workout_name, workout_description, day_idx, session_opt) = match &detail {
        Some(d) => (
            d.program_name.clone(),
            d.workout_name.clone(),
            d.workout_description.clone(),
            d.day_index,
            d.session.clone(),
        ),
        None => (String::new(), String::new(), None, day_index_val, None),
    };

    let feedback_completed = session_opt
        .as_ref()
        .map(|s| s.completed_at.is_some())
        .unwrap_or(false);
    let feedback_sid = session_opt.as_ref().map(|s| s.id.clone());
    let exercises_for_detail = detail
        .as_ref()
        .map(|d| d.exercises.clone())
        .unwrap_or_default();
    let exercise_row_data: Vec<(String, String, Option<String>, Option<String>, i32, i32)> =
        exercises_for_detail
            .iter()
            .map(|we| {
                (
                    we.exercise.id.clone(),
                    we.exercise.name.clone(),
                    we.exercise.description.clone(),
                    we.exercise.video_url.clone(),
                    we.sets,
                    we.reps,
                )
            })
            .collect();

    let exercise_rows: Vec<dioxus::prelude::Element> = exercise_row_data
        .into_iter()
        .map(|(ex_id, ex_name, ex_desc, ex_video_url, sets, reps)| {
            let ex_id_effort = ex_id.clone();
            let ex_id_pain = ex_id.clone();
            let ex_id_comment = ex_id.clone();
            let embed_url = ex_video_url.as_ref().and_then(|u| {
                // Extrae el ID de YouTube y construye la URL embebida.
                if let Some(pos) = u.find("v=") {
                    let id = u[pos + 2..]
                        .split('&')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !id.is_empty() {
                        return Some(format!("https://www.youtube.com/embed/{}", id));
                    }
                }
                if let Some(pos) = u.rfind('/') {
                    let id = u[pos + 1..]
                        .split(&['?', '&'][..])
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !id.is_empty() {
                        return Some(format!("https://www.youtube.com/embed/{}", id));
                    }
                }
                None
            });
            let effort = exercise_feedback().get(&ex_id).map(|t| t.0 as f64).unwrap_or(1.0);
            let pain = exercise_feedback().get(&ex_id).map(|t| t.1 as f64).unwrap_or(0.0);
            let comment = exercise_feedback().get(&ex_id).map(|t| t.2.clone()).unwrap_or_default();
            log::info!("DEBUG ex_id: {ex_id}, effort: {effort}, pain: {pain}, comment: {comment}");
            rsx! {
                div { class: "mb-4 p-3 rounded-md border border-border",
                    key: "{ex_id}",
                    p { class: "font-medium mb-2", "{ex_name}" }
                    if let Some(desc) = ex_desc.clone() {
                        if !desc.is_empty() {
                            p { class: "text-sm text-text-muted mb-2", "{desc}" }
                        }
                    }
                    p { class: "text-sm text-text-muted mb-2", "Series: {sets} × Repeticiones: {reps}" }
                    if let Some(embed) = embed_url.clone() {
                        iframe {
                            class: "w-full mb-3 aspect-video rounded-md border border-border bg-black",
                            src: "{embed}",
                            allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                            allowfullscreen: "true",
                        }
                    }
                    div {
                        div { class: "flex items-start justify-start gap-4",
                            label { class: "text-sm font-semibold mt-0 mb-0 w-1/4", "Esfuerzo" }
                            Slider {
                                min: 1.0,
                                max: 10.0,
                                step: 1.0,
                                horizontal: true,
                                default_value: SliderValue::Single(effort),
                                value: Some(SliderValue::Single(effort)),
                                on_value_change: move |value: SliderValue| {
                                    let SliderValue::Single(v) = value;
                                    let mut m = exercise_feedback();
                                    let entry = m.entry(ex_id_effort.clone()).or_insert((v as i32, pain as i32, String::new()));
                                    entry.0 = v as i32;
                                    exercise_feedback.set(m);
                                },
                                SliderTrack {
                                    SliderRange { }
                                    SliderThumb {}
                                }
                            }
                            span { class: "text-sm font-semibold mb-4", "{effort}" }
                        }
                        div { class: "flex items-start justify-start gap-4",
                            label { class: "text-sm font-semibold mt-0 mb-0 w-1/4", "Dolor" }
                            Slider {
                                min: 0.0,
                                max: 10.0,
                                step: 1.0,
                                horizontal: true,
                                default_value: SliderValue::Single(pain),
                                value: Some(SliderValue::Single(pain)),
                                on_value_change: move |value: SliderValue| {
                                    let SliderValue::Single(v) = value;
                                    let mut m = exercise_feedback();
                                    let entry = m.entry(ex_id_pain.clone()).or_insert((effort as i32, v as i32, String::new()));
                                    entry.1 = v as i32;
                                    exercise_feedback.set(m);
                                },
                                SliderTrack {
                                    SliderRange { }
                                    SliderThumb {}
                                }
                            }
                            span { class: "text-sm font-semibold mb-4", "{pain}" }
                        }
                        label { class: "text-sm font-semibold mt-0 mb-0", for: "comment-{ex_id}", "Comentario" }
                        Textarea {
                            id: "comment-{ex_id}",
                            variant: TextareaVariant::Outline,
                            placeholder: "Opcional",
                            value: "{comment}",
                            oninput: move |e: FormEvent| {
                                let mut m = exercise_feedback();
                                let entry = m.entry(ex_id_comment.clone()).or_insert((effort as i32, pain as i32, e.value().clone()));
                                entry.2 = e.value().clone();
                                exercise_feedback.set(m);
                            },
                        }
                    }
                }
            }
            .into()
        })
        .collect();
    let feedback_sid_submit = feedback_sid.clone();
    let feedback_sid_uncomplete = feedback_sid.clone();

    let backend_submit = backend.clone();
    let backend_uncomplete = backend.clone();
    let session_signal_clone = session_signal.clone();
    let data_clone = data.clone();

    rsx! {
        div {
            class: "view container mx-auto patient-workout-day flex items-center justify-center",
            div {
                class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                Backview {
                    to: Route::PatientDashboard {},
                    if !program_name.is_empty() {
                        "{program_name}"
                    } else {
                        "Mi programa"
                    }
                }
                if let Some(err) = data.read().as_ref().and_then(|r| r.as_ref().err()).cloned() {
                    p { class: "text-error", "{err}" }
                } else if detail.is_none() {
                    p { "Cargando..." }
                } else {
                    h2 { class: "text-xl font-semibold mb-1", "{workout_name}" }
                    if let Some(desc) = workout_description.clone() {
                        if !desc.is_empty() {
                            p { class: "text-sm text-text-muted mb-3", "{desc}" }
                        }
                    }
                    section {
                        if exercise_rows.is_empty() {
                            p { class: "text-sm text-text-muted", "Este entrenamiento no tiene ejercicios configurados." }
                        } else {
                            {exercise_rows.into_iter()}
                        }
                        section { class: "bg-surface rounded-lg p-4 mb-6 border border-border flex flex-wrap items-center gap-2",
                            p { class: "text-medium font-semibold mt-0 mb-0", "Completado el" }
                            input {
                                class: "min-h-11 px-3 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                                r#type: "date",
                                value: "{session_date()}",
                                oninput: move |ev| session_date.set(ev.value().clone()),
                            }
                        }
                        button {
                            class: "mt-4 min-h-11 px-4 rounded-md bg-primary text-white font-medium",
                            disabled: submit_loading(),
                            onclick: move |_| {
                                let date_str = session_date().clone();
                                let backend = backend_submit.clone();
                                let sess = session_signal_clone.read().clone();
                                let Some(sess) = sess else { return };
                                let token = sess.access_token().to_string();
                                let pid = detail.as_ref().map(|d| d.patient_program_id.clone()).unwrap_or_default();
                                let di = day_idx;
                                let sid_opt = feedback_sid_submit.clone();
                                let completed = feedback_completed;
                                let fb_map = exercise_feedback();
                                let exercises_list = exercises_for_detail.clone();
                                submit_loading.set(true);
                                submit_error.set(None);
                                let mut data = data_clone.clone();
                                spawn(async move {
                                    let sid = match sid_opt.as_ref() {
                                        Some(s) => s.clone(),
                                        None => {
                                            match backend.get_or_create_session(&token, &pid, di, &date_str).await {
                                                Ok(s) => s.id,
                                                Err(err) => {
                                                    submit_error.set(Some(err));
                                                    submit_loading.set(false);
                                                    return;
                                                }
                                            }
                                        }
                                    };
                                    if let Err(e) = backend.update_session(
                                        &token,
                                        &sid,
                                        Some(&date_str),
                                    ).await {
                                        submit_error.set(Some(e));
                                        submit_loading.set(false);
                                        return;
                                    }
                                    if !completed {
                                        if let Err(e) = backend.complete_session(&token, &sid).await {
                                            submit_error.set(Some(e));
                                            submit_loading.set(false);
                                            return;
                                        }
                                    }
                                    for we in &exercises_list {
                                        let (eff, pa, com) = fb_map.get(&we.exercise.id).cloned().unwrap_or((5, 0, String::new()));
                                        if let Err(e) = backend.upsert_session_exercise_feedback(
                                            &token,
                                            &sid,
                                            &we.exercise.id,
                                            Some(eff),
                                            Some(pa),
                                            if com.is_empty() { None } else { Some(com.as_str()) },
                                        ).await {
                                            submit_error.set(Some(e));
                                            break;
                                        }
                                    }
                                    data.restart();
                                    submit_loading.set(false);
                                });
                            },
                            if feedback_completed { "Guardar cambios" } else { "Marcar completada y enviar feedback" }
                        }
                        if feedback_completed {
                            button {
                                class: "mt-2 ml-2 bg-transparent text-primary underline min-h-0 py-1",
                                disabled: submit_loading(),
                                onclick: move |_| {
                                    let Some(ref session_id) = feedback_sid_uncomplete else { return };
                                    let backend = backend_uncomplete.clone();
                                    let sess = session_signal_clone.read().clone();
                                    let Some(sess) = sess else { return };
                                    let token = sess.access_token().to_string();
                                    let session_id = session_id.clone();
                                    let mut data = data.clone();
                                    submit_loading.set(true);
                                    submit_error.set(None);
                                    spawn(async move {
                                        let res = backend.uncomplete_session(&token, &session_id).await;
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
                        if let Some(ref e) = *submit_error.read() {
                            p { class: "text-error text-sm mt-2", "{e}" }
                        }
                    }
                }
            }
        }
    }
}
