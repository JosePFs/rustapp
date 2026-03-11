use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::Route;

use crate::infrastructure::ui::components::{
    Agenda, Card, CardContent, CardDescription, CardHeader, CardTitle, SkeletonCard,
};
use crate::infrastructure::ui::hooks::patient_programs::use_patient_programs;
use crate::infrastructure::ui::hooks::AsyncState;

#[component]
pub fn PatientDashboard() -> Element {
    let navigator = use_navigator();
    let patient_programs = use_patient_programs();
    let mut selected_for_feedback = use_signal(|| Option::<(String, i32)>::None);

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

    let programs_content = match &*patient_programs.state.read() {
        AsyncState::Loading => rsx! {
            SkeletonCard { }
        },
        AsyncState::Error(_) => rsx! {
            p { class: "text-text-muted italic", { t!("error_programs") } }
            Link { to: Route::LoginView {}, { t!("login_title") } }
        },
        AsyncState::Ready(data) => {
            if data.patient_programs.is_empty() {
                rsx! {
                    p { class: "text-text-muted italic", { t!("no_programs_assigned") } }
                }
            } else {
                rsx! {
                    for prog in data.patient_programs.iter() {
                        section { class: "mb-4",
                            Card {
                                CardHeader {
                                    CardTitle {
                                        "{prog.program_name}"
                                    }
                                    if let Some(ref desc) = prog.program_description {
                                        CardDescription {
                                            "{desc}"
                                        }
                                    }
                                }
                                CardContent {
                                    Agenda {
                                        sessions: prog.sessions.clone(),
                                        program_feedback: prog.program_feedback.clone(),
                                        schedule: prog.schedule.clone(),
                                        workouts: prog.workouts.clone(),
                                        title: t!("progress"),
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
    };

    rsx! {
        div { class: "view container mx-auto patient-dashboard",
            div { class: "content min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                div { class: "flex items-center justify-between mb-4",
                    h1 { class: "text-2xl font-semibold", { t!("patient_dashboard_title") } }
                }
                div { class: "flex flex-col",
                    { programs_content }
                }
            }
        }
    }
}
