use dioxus::prelude::*;

use dioxus_i18n::t;
use dioxus_router::use_navigator;

use crate::Route;

use crate::infrastructure::ui::components::AgendaBlock;
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
            p { class: "text-text-muted italic", { t!("loading_programs") } }
        },
        AsyncState::Error(_) => rsx! {
            p { class: "text-text-muted italic", { t!("error_programs") } }
        },
        AsyncState::Ready(data) => {
            if data.patient_programs.is_empty() {
                rsx! {
                    p { class: "text-text-muted italic", { t!("no_programs_assigned") } }
                }
            } else {
                rsx! {
                    for prog in data.patient_programs.iter() {
                        section { key: "{prog.patient_program_id}", class: "bg-surface border border-border rounded-md p-4 mb-4",
                            h2 { class: "text-xl font-semibold mt-0 mb-2", "{prog.program_name}" }
                            if let Some(ref desc) = prog.program_description {
                                p { class: "text-sm text-text-muted mb-4", "{desc}" }
                            }
                            AgendaBlock {
                                sessions: prog.sessions.clone(),
                                program_feedback: prog.program_feedback.clone(),
                                schedule: prog.schedule.clone(),
                                workouts: prog.workouts.clone(),
                                title: "Progreso".to_string(),
                                patient_program_id: Some(prog.patient_program_id.clone()),
                                write_selected_for_feedback: Some(selected_for_feedback),
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "view container mx-auto patient-dashboard flex items-center justify-center",
            div { class: "content pt-2 min-w-[280px] sm:min-w-[320px] md:min-w-[400px] lg:min-w-2xl",
                div { class: "flex items-center justify-between mb-6",
                    h1 { class: "text-2xl font-semibold", "Mis programas" }
                }
                { programs_content }
            }
        }
    }
}
