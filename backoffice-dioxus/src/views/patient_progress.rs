use dioxus::prelude::*;
use dioxus_i18n::t;
use dioxus_router::Link;

use crate::components::Agenda;
use crate::hooks::{patient_progress::use_patient_progress, AsyncState};
use crate::Route;
use application::use_cases::agenda_schedule::AgendaSessionFeedback;
use application::use_cases::patient_progress::PatientProgressResult;

#[component]
pub fn PatientProgress(id: String) -> Element {
    let progress = use_patient_progress(id.clone());

    rsx! {
        div {
            class: "view container mx-auto patient-progress w-full",
            div {
                class: "content w-full",
                match &*progress.state.read() {
                    AsyncState::Idle | AsyncState::Loading => {
                        rsx! { p { { t!("loading") } } }
                    }
                    AsyncState::Error(_) => {
                        rsx! { p { class: "text-error", { t!("error_patient_progress") } } }
                    }
                    AsyncState::Ready(PatientProgressResult { profile, programs_with_sessions }) => rsx! {
                        h1 { class: "text-2xl font-semibold mb-4", { t!("patient_progress_of", name: profile.full_name.clone()) } }
                        p { class: "text-sm text-text-muted mb-4", "{profile.email}" }
                        if programs_with_sessions.is_empty() {
                            p { class: "text-text-muted italic py-4", { t!("patient_no_programs_assigned") } }
                        } else {
                            for pws in programs_with_sessions.iter() {
                                section {
                                    class: "bg-surface rounded-lg p-4 mb-6 shadow-sm border border-border",
                                    h2 { class: "text-xl font-semibold mt-0 mb-2", "{pws.program_name}" }
                                    if let Some(ref desc) = pws.program_description {
                                        p { class: "text-sm text-text-muted mb-2", "{desc}" }
                                    }
                                    p { class: "text-xs text-text-muted mb-2", { t!("patient_program_status", status: pws.assignment_status.clone()) } }
                                    Agenda {
                                        sessions: pws.sessions.clone(),
                                        program_feedback: pws.program_feedback.clone(),
                                        schedule: pws.schedule.clone(),
                                        workouts: pws.workouts.clone(),
                                        title: t!("progress"),
                                        patient_program_id: None,
                                        write_selected_for_feedback: None,
                                    }
                                    if pws.sessions.is_empty() {
                                        p { class: "text-text-muted italic py-4", { t!("patient_no_sessions") } }
                                    } else {
                                        div { class: "overflow-x-auto",
                                            table { class: "border-collapse text-sm w-full whitespace-nowrap",
                                                thead {
                                                    tr {
                                                        th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", { t!("patient_progress_day") } }
                                                        th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", { t!("patient_progress_date") } }
                                                        th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", { t!("patient_progress_completed") } }
                                                        th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", { t!("patient_progress_effort_avg") } }
                                                        th { class: "text-left p-2 font-semibold text-text-muted border-b border-border", { t!("patient_progress_pain_avg") } }
                                                    }
                                                }
                                                tbody {
                                                    for s in pws.sessions.iter() {
                                                        tr { key: "{s.id}", class: "border-b border-border",
                                                            td { class: "p-2", { t!("patient_progress_day_label", day: (s.day_index + 1).to_string()) } }
                                                            td { class: "p-2", "{s.session_date}" }
                                                            td { class: "p-2", { if s.completed_at.is_some() { t!("yes") } else { t!("no") } } }
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
                    }
                }
            }
        }
    }
}

const EMPTY: &str = "";

fn session_avg_feedback(session_id: &str, feedback: &[AgendaSessionFeedback]) -> (String, String) {
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
