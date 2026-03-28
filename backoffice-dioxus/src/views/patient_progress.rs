use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::hooks::{
    patient_progress::{use_patient_progress, PatientProgressResult},
    AsyncState,
};

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
                                    p { class: "text-text-muted italic py-4", { t!("patient_no_sessions") } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
