use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::Link;

use crate::components::{
    Backview, Button, ButtonVariant, Card, CardContent, PatientWorkout, SkeletonCard,
};
use crate::hooks::app_context::use_app_context;
use crate::hooks::{
    submit_workout_feedback::use_submit_workout_feedback,
    uncomplete_workout_session::use_uncomplete_workout_session,
    workout_day_detail::use_workout_day_detail,
};
use crate::Route;

#[component]
pub fn PatientWorkoutSessionView(patient_program_id: String, day_index: String) -> Element {
    let app_context = use_app_context();
    let session_signal = app_context.session();

    let day_index_val = day_index.parse::<i32>().unwrap_or(0);
    let workout_day_detail = use_workout_day_detail(patient_program_id.clone(), day_index_val);

    let mut session_date = use_signal(|| chrono::Utc::now().format("%Y-%m-%d").to_string());
    let mut exercise_feedback = use_signal(|| HashMap::<String, (i32, i32, String)>::new());
    let mut submit_error = use_signal(|| Option::<String>::None);
    let mut marked_as_completed = use_signal(|| false);

    use_effect(move || {
        let detail = workout_day_detail
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok().cloned());

        let Some(d) = detail else {
            return;
        };

        if let Some(ref sess) = d.session {
            session_date.set(sess.session_date.clone());
        } else {
            session_date.set(chrono::Utc::now().format("%Y-%m-%d").to_string());
        }

        let feedback_completed = d
            .session
            .as_ref()
            .map(|ws| ws.completed_at.is_some())
            .unwrap_or(false);

        marked_as_completed.set(feedback_completed);

        let mut map = HashMap::new();
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
    });

    let session = session_signal.read().clone();
    if session.is_none() {
        return rsx! {
            div {
                class: "view container mx-auto patient-workout-day",
                div {
                    class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                    div {
                        { t!("must_login_message") }
                        " "
                        Link { to: Route::LoginView {}, { t!("go_to_login") } }
                    }
                }
            }
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

    let feedback_sid = session_opt.as_ref().map(|s| s.id.clone());
    let exercises_for_detail = detail
        .as_ref()
        .map(|d| d.exercises.clone())
        .unwrap_or_default();
    let exercise_row_data: Vec<(String, String, Option<String>, Option<String>, i32, i32)> =
        exercises_for_detail
            .iter()
            .map(|we| {
                let embed_url = we.exercise.video_url.as_ref().and_then(|u| {
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
                (
                    we.exercise.id.clone(),
                    we.exercise.name.clone(),
                    we.exercise.description.clone(),
                    embed_url,
                    we.sets,
                    we.reps,
                )
            })
            .collect();

    let feedback_sid_uncomplete = feedback_sid.clone();

    let mut submit_feedback = use_submit_workout_feedback(
        detail
            .as_ref()
            .map(|d| d.patient_program_id.clone())
            .unwrap_or_default(),
        day_idx,
        session_date,
        exercise_feedback,
        marked_as_completed.peek().cloned(),
    );

    let mut uncomplete_workout_session = use_uncomplete_workout_session();

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
                        if exercise_row_data.is_empty() {
                            p { class: "text-sm text-text-muted", { t!("workout_no_exercises") } }
                        } else {
                            for (ex_id, ex_name, ex_desc, embed_url, sets, reps) in exercise_row_data.iter() {
                                PatientWorkout {
                                    key: "{ex_id}",
                                    exercise_id: ex_id.clone(),
                                    exercise_name: ex_name.clone(),
                                    exercise_desc: ex_desc.clone(),
                                    embed_url: embed_url.clone(),
                                    sets: *sets,
                                    reps: *reps,
                                    exercise_feedback,
                                }
                            }
                        }
                        section { class: "mb-4",
                            Card {
                                CardContent {
                                    div { class: "flex flex-row items-center justify-start gap-2",
                                        p { class: "text-medium font-semibold mt-0 mb-0", { t!("completed_on") } }
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
                                class: if submit_feedback.state.read().is_loading() {
                                    "opacity-50 !cursor-not-allowed"
                                } else {
                                    ""
                                },
                                disabled: submit_feedback.state.read().is_loading(),
                                onclick: move |_| {
                                    submit_error.set(None);
                                    submit_feedback.action.call(());
                                    marked_as_completed.set(true);
                                },
                                if marked_as_completed() {
                                    { t!("save_changes") }
                                } else {
                                    { t!("mark_completed_and_send_feedback") }
                                }
                            }
                            if marked_as_completed() {
                                Button {
                                    class: if uncomplete_workout_session.state.read().is_loading() {
                                        "opacity-50 !cursor-not-allowed mt-6 mb-4"
                                    } else {
                                        "mt-6 mb-4"
                                    },
                                    variant: ButtonVariant::Outline,
                                    disabled: uncomplete_workout_session.state.read().is_loading(),
                                    onclick: move |_| {
                                        let Some(ref session_id) = feedback_sid_uncomplete else { return };
                                        let session_id = session_id.clone();
                                        submit_error.set(None);
                                        uncomplete_workout_session.action.call(session_id);
                                        marked_as_completed.set(false);
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
