use dioxus::prelude::*;
use std::collections::HashMap;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::PatientProgressArgs;

#[derive(Clone, Debug)]
pub struct PatientProgressProfile {
    pub full_name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct PatientProgressProgramBlock {
    pub program_name: String,
    pub program_description: Option<String>,
    pub assignment_status: String,
    pub sessions: Vec<()>,
    pub program_feedback: HashMap<String, ()>,
    pub schedule: Vec<()>,
    pub workouts: HashMap<String, ()>,
}

#[derive(Clone, Debug)]
pub struct PatientProgressResult {
    pub profile: PatientProgressProfile,
    pub programs_with_sessions: Vec<PatientProgressProgramBlock>,
}

impl Default for PatientProgressResult {
    fn default() -> Self {
        Self {
            profile: PatientProgressProfile {
                full_name: String::new(),
                email: String::new(),
            },
            programs_with_sessions: vec![],
        }
    }
}

#[derive(Clone)]
pub struct UsePatientProgress {
    pub resource: Resource<PatientProgressResult>,
    pub state: Signal<AsyncState<PatientProgressResult>>,
}

pub fn use_patient_progress(patient_id: String) -> UsePatientProgress {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<PatientProgressResult>::Idle);

    let facade = facade.clone();

    let resource = use_resource(move || {
        let facade = facade.clone();
        let patient_id = patient_id.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = PatientProgressArgs { patient_id };
            match facade.patient_progress(args).await {
                Ok(data) => {
                    let result = PatientProgressResult {
                        profile: PatientProgressProfile {
                            full_name: data.profile.full_name,
                            email: data.profile.email,
                        },
                        programs_with_sessions: data
                            .programs_with_sessions
                            .into_iter()
                            .map(|p| PatientProgressProgramBlock {
                                program_name: p.program_name,
                                program_description: p.program_description,
                                assignment_status: p.assignment_status,
                                sessions: vec![],
                                program_feedback: HashMap::new(),
                                schedule: vec![],
                                workouts: HashMap::new(),
                            })
                            .collect(),
                    };
                    state.set(AsyncState::Ready(result.clone()));
                    result
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    PatientProgressResult::default()
                }
            }
        }
    });

    UsePatientProgress { state, resource }
}
