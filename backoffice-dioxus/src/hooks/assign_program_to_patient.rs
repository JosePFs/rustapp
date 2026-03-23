use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::assign_program_to_patient::AssignProgramToPatientArgs;

#[derive(Clone)]
pub struct UseAssignProgramToPatient {
    pub action: Action<((Vec<String>, Vec<String>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_assign_program_to_patient() -> UseAssignProgramToPatient {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(
        move |(patient_ids, program_ids): (Vec<String>, Vec<String>)| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let mut any_error = None;
                for patient_id in patient_ids.iter() {
                    for program_id in program_ids.iter() {
                        let args = AssignProgramToPatientArgs {
                            patient_id: patient_id.clone(),
                            program_id: program_id.clone(),
                        };
                        if let Err(e) = facade.assign_program_to_patient(args).await {
                            any_error = Some(e);
                            break;
                        }
                    }
                    if any_error.is_some() {
                        break;
                    }
                }

                match any_error {
                    Some(e) => {
                        state.set(AsyncState::Error(e.clone()));
                        Err(e)
                    }
                    None => {
                        state.set(AsyncState::Ready(()));
                        Ok(())
                    }
                }
            }
        },
    );

    UseAssignProgramToPatient { action, state }
}
