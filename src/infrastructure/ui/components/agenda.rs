use dioxus::prelude::*;

use crate::domain::entities::{ProgramScheduleItem, Workout, WorkoutSession};
use crate::infrastructure::supabase::api::build_agenda_schedule;

/// When provided, clicking a slot writes (patient_program_id, day_index) so the parent can show the feedback form.
#[component]
pub fn AgendaBlock(
    sessions: Vec<WorkoutSession>,
    schedule: Vec<ProgramScheduleItem>,
    workouts: Vec<Workout>,
    title: String,
    patient_program_id: Option<String>,
    write_selected_for_feedback: Option<Signal<Option<(String, i32)>>>,
) -> Element {
    let mut selected_day_index = use_signal(|| Option::<i32>::None);

    let day_schedule = build_agenda_schedule(&schedule, &workouts);
    let training_day_indexes: std::collections::HashSet<i32> = day_schedule
        .iter()
        .filter(|(_, wid, _)| wid.is_some())
        .map(|(idx, _, _)| *idx)
        .collect();
    let total_training_days = training_day_indexes.len().max(1);
    let completed_count = sessions
        .iter()
        .filter(|s| s.completed_at.is_some() && training_day_indexes.contains(&s.day_index))
        .count();
    let pct = if total_training_days > 0 {
        completed_count as f64 / total_training_days as f64 * 100.0
    } else {
        0.0
    };
    let (avg_effort_str, avg_pain_str) = {
        let efforts: Vec<i32> = sessions.iter().filter_map(|s| s.effort).collect();
        let pains: Vec<i32> = sessions.iter().filter_map(|s| s.pain).collect();
        let e = if efforts.is_empty() {
            String::new()
        } else {
            format!(
                "{:.1}",
                efforts.iter().sum::<i32>() as f64 / efforts.len() as f64
            )
        };
        let p = if pains.is_empty() {
            String::new()
        } else {
            format!(
                "{:.1}",
                pains.iter().sum::<i32>() as f64 / pains.len() as f64
            )
        };
        (e, p)
    };

    let sessions_by_day: std::collections::HashMap<i32, WorkoutSession> =
        sessions.iter().map(|s| (s.day_index, s.clone())).collect();

    let day_rows: Vec<Element> = day_schedule
        .iter()
        .map(|(day_idx, wid_opt, day_label)| {
            let is_training_day = wid_opt.is_some();
            let session = sessions_by_day.get(day_idx);
            let completed = session.map(|s| s.completed_at.is_some()).unwrap_or(false);
            let is_selected = selected_day_index() == Some(*day_idx);
            let mut sid = selected_day_index;
            let idx = *day_idx;
            let type_label = day_label.clone();
            let pid_opt = patient_program_id.clone();
            let write_signal = write_selected_for_feedback;
            let day_num = idx + 1;
            if is_training_day {
                rsx! {
                    li { key: "{idx}",
                        button {
                            class: if is_selected { "block w-full text-left p-4 min-h-11 border border-primary rounded-md bg-green-50 font-medium text-sm" } else { "block w-full text-left p-4 min-h-11 border border-border rounded-md bg-surface text-sm hover:border-primary hover:bg-gray-50" },
                            onclick: move |_| {
                                let current = sid();
                                let new_sid = if current == Some(idx) { None } else { Some(idx) };
                                sid.set(new_sid);
                                if let Some(mut sig) = write_signal {
                                    if let Some(new_idx) = new_sid {
                                        if let Some(pid) = pid_opt.as_ref() {
                                            sig.set(Some((pid.to_string(), new_idx)));
                                        }
                                    } else {
                                        sig.set(None);
                                    }
                                }
                            },
                            span { class: "font-medium", "Día {day_num} - " }
                            span { "{type_label}" }
                            if completed {
                                span { class: "text-success ml-1", "✓" }
                            } else {
                                span { class: "text-text-muted ml-1", "—" }
                            }
                        }
                    }
                }
                .into()
            } else {
                rsx! {
                    li { key: "{idx}",
                        span { class: "block py-2 px-4 text-sm text-text-muted italic",
                            "Día {day_num} "
                            "{type_label} —"
                        }
                    }
                }
                .into()
            }
        })
        .collect();

    let selected_idx = selected_day_index();
    let detail = selected_idx.and_then(|idx| sessions_by_day.get(&idx).cloned());
    let detail_with_label = detail.as_ref().map(|sess| {
        let label = if sess.completed_at.is_some() {
            "Sí"
        } else {
            "No"
        };
        (sess.clone(), label)
    });

    let detail_block: Option<dioxus::prelude::Element> = if let Some((
        ref sess,
        ref completed_label,
    )) = detail_with_label
    {
        let sess = sess.clone();
        let completed_label = (*completed_label).to_string();
        Some(rsx! {
            div { class: "p-4 bg-gray-50 rounded-md mb-4 text-sm",
                h3 { class: "text-base font-semibold mt-0 mb-2", "Día {sess.day_index + 1} — Completada: {completed_label}" }
                p { class: "text-text-muted", "Fecha registrada: {sess.session_date}" }
                if sess.effort.is_some() || sess.pain.is_some() || sess.comment.as_deref().unwrap_or("").len() > 0 {
                    p { "Esfuerzo: {sess.effort.map(|e| e.to_string()).unwrap_or_default()}" }
                    p { "Dolor: {sess.pain.map(|p| p.to_string()).unwrap_or_default()}" }
                    if let Some(ref c) = sess.comment {
                        if !c.is_empty() {
                            p { "Comentario: {c}" }
                        }
                    }
                } else {
                    p { class: "text-text-muted", "Sin feedback registrado." }
                }
                button {
                    class: "bg-transparent text-primary underline min-h-0 py-1 mt-2",
                    onclick: move |_| selected_day_index.set(None),
                    "Cerrar"
                }
            }
        }.into())
    } else if selected_idx.is_some() {
        Some(rsx! {
            div { class: "p-4 bg-gray-50 rounded-md mb-4 text-sm",
                p { class: "text-text-muted", "Día de entrenamiento sin sesión registrada. Usa el formulario de feedback más abajo para marcar y enviar." }
                button {
                    class: "bg-transparent text-primary underline min-h-0 py-1 mt-2",
                    onclick: move |_| selected_day_index.set(None),
                    "Cerrar"
                }
            }
        }.into())
    } else {
        None
    };

    rsx! {
        section { class: "mb-6",
            h2 { class: "text-xl font-semibold mb-2", "{title}" }
            div { class: "text-sm text-text-muted mb-4",
                "Días completados: {pct:.0}%"
                span { class: "mx-1", " | " }
                "Esfuerzo medio: {avg_effort_str}"
                span { class: "mx-1", " | " }
                "Dolor medio: {avg_pain_str}"
            }
            ul { class: "list-none p-0 m-0",
                {day_rows.into_iter()}
            }
            if let Some(block) = detail_block {
                {block}
            }
        }
    }
}
