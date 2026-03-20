use dioxus::prelude::*;

use crate::app_context::AssignProgramToPatientUseCaseType;
use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::assign_program_to_patient::{
    AssignProgramToPatientArgs, AssignProgramToPatientUseCase,
};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseAssignProgramToPatient {
    pub action: Action<((Vec<String>, Vec<String>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_assign_program_to_patient() -> UseAssignProgramToPatient {
    let app_context = use_app_context();
    let use_case = app_context.use_case::<AssignProgramToPatientUseCaseType>();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(patient_ids, program_ids): (Vec<String>, Vec<String>)| {
            let use_case = use_case_for_action.clone();
            let session_signal = session_signal_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let sess_opt = session_signal.read().clone();
                let Some(sess) = sess_opt else {
                    state.set(AsyncState::Error(DomainError::SessionNotFound));
                    return Err(DomainError::SessionNotFound);
                };

                let token = sess.access_token().to_string();

                let mut any_error = None;
                for patient_id in patient_ids.iter() {
                    for program_id in program_ids.iter() {
                        let args = AssignProgramToPatientArgs {
                            token: token.clone(),
                            patient_id: patient_id.clone(),
                            program_id: program_id.clone(),
                        };
                        if let Err(e) = use_case.execute(args).await {
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
