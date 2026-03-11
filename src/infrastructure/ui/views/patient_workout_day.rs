//! Página de detalle de un día de entrenamiento para el paciente:
//! muestra los ejercicios del entrenamiento y permite guardar/editar el feedback.

use dioxus::prelude::*;

use dioxus_primitives::slider::SliderValue;
use dioxus_router::Link;
use dioxus_i18n::t;

use crate::infrastructure::app_context::AppContext;
use crate::infrastructure::ui::components::{
    Backview, Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle, SkeletonCard, Slider, SliderRange, SliderThumb, SliderTrack, Textarea, TextareaVariant
};
use crate::infrastructure::ui::hooks::workout_day_detail::use_workout_day_detail;
use crate::Route;

#[component]
pub fn PatientWorkoutDay(patient_program_id: String, day_index: String) -> Element {
    let app_context = use_context::<AppContext>();
    let backend = app_context.backend();
    let session_signal = app_context.session();

    let day_index_val = day_index.parse::<i32>().unwrap_or(0);
    let workout_day_detail = use_workout_day_detail(
        session_signal.read().clone(),
        backend.clone(),
        patient_program_id.clone(),
        day_index_val,
    );

    let mut session_date = use_signal(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let mut exercise_feedback =
        use_signal(|| std::collections::HashMap::<String, (i32, i32, String)>::new());
    let mut submit_loading = use_signal(|| false);
    let mut submit_error = use_signal(|| Option::<String>::None);

    use_effect(move || {
        let detail = workout_day_detail
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok().cloned());

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

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div { "Debes iniciar sesión. " Link { to: Route::LoginView {}, "Ir a login" } }
        };
    }

    let detail = workout_day_detail
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok().cloned());

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
            rsx! {
                article { class: "mb-4",
                    Card {
                    key: "{ex_id}",
                    CardHeader {
                        CardTitle {
                            "{ex_name}"
                        }
                        if let Some(desc) = ex_desc.clone() {
                            if !desc.is_empty() {
                                CardDescription {
                                    "{desc}"
                                }
                            }
                        }
                    }
                    CardContent {
                        p { class: "text-sm font-semibold text-text-muted mb-2", "Series: {sets} × Repeticiones: {reps}" }
                        if let Some(embed) = embed_url.clone() {
                            iframe {
                                class: "w-full mb-6 aspect-video rounded-md border border-border bg-black",
                                src: "{embed}",
                                allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                                allowfullscreen: "true",
                            }
                        }
                        div {
                            div { class: "flex items-start justify-start gap-4",
                                label { class: "text-sm mt-0 mb-0 w-1/4", "Esfuerzo" }
                                Slider {
                                    min: 1.0,
                                    max: 10.0,
                                    step: 1.0,
                                    horizontal: true,
                                    default_value: SliderValue::Single(effort),
                                    value: Some(SliderValue::Single(effort)),
                                    on_value_change: move |value: SliderValue| {
                                        let SliderValue::Single(v) = value;
                                        let mut m = exercise_feedback.peek().clone();
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
                                label { class: "text-sm mt-0 mb-0 w-1/4", "Dolor" }
                                Slider {
                                    min: 0.0,
                                    max: 10.0,
                                    step: 1.0,
                                    horizontal: true,
                                    default_value: SliderValue::Single(pain),
                                    value: Some(SliderValue::Single(pain)),
                                    on_value_change: move |value: SliderValue| {
                                        let SliderValue::Single(v) = value;
                                        let mut m = exercise_feedback.peek().clone();
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
                            label { class: "text-sm mt-0 mb-0", for: "comment-{ex_id}", "Comentario" }
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
    let data_clone = workout_day_detail.clone();

    rsx! {
        div {
            class: "view container mx-auto patient-workout-day",
            div {
                class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                Backview {
                    to: Route::PatientDashboard {},
                    if !program_name.is_empty() {
                        "{program_name}"
                    } else {
                        { t!("patient_dashboard_title") }
                    }
                }
                if let Some(err) = workout_day_detail.read().as_ref().and_then(|r| r.as_ref().err()).cloned() {
                    p { class: "text-error", "{err}" }
                } else if detail.is_none() {
                    SkeletonCard { }
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
                        section { class: "mb-4",
                            Card {
                                CardContent {
                                    div { class: "flex flex-row items-center justify-start gap-2",
                                        p { class: "text-medium font-semibold mt-0 mb-0", "Completado el" }
                                        input {
                                            class: "min-h-11 px-3 border border-border rounded-md bg-surface focus:outline-none focus:border-primary",
                                            r#type: "date",
                                            value: "{session_date()}",
                                            oninput: move |ev| session_date.set(ev.value().clone()),
                                        }
                                    }
                                }
                            }
                        }
                        section { class: "mb-6",
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
                                Button { class: "mt-6 mb-4",
                                    variant: ButtonVariant::Outline,
                                    disabled: submit_loading(),
                                    onclick: move |_| {
                                        let Some(ref session_id) = feedback_sid_uncomplete else { return };
                                        let backend = backend_uncomplete.clone();
                                        let sess = session_signal_clone.read().clone();
                                        let Some(sess) = sess else { return };
                                        let token = sess.access_token().to_string();
                                        let session_id = session_id.clone();
                                        let mut data = workout_day_detail.clone();
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
                                    { t!("mark_as_uncompleted") }
                                }
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
