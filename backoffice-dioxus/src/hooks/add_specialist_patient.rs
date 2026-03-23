use application::ports::error::ApplicationError;
use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::add_specialist_patient::AddSpecialistPatientArgs;

#[derive(Clone)]
pub struct UseAddSpecialistPatient {
    pub action: Action<(String,), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_add_specialist_patient() -> UseAddSpecialistPatient {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(move |patient_email: String| {
        let facade = facade_for_action.clone();
        let session_signal = session_signal_for_action.clone();
        let mut state = state.clone();

        state.set(AsyncState::Loading);

        async move {
            let sess_opt = session_signal.read().clone();
            let Some(sess) = sess_opt else {
                state.set(AsyncState::Error(ApplicationError::NoSession));
                return Err(ApplicationError::NoSession);
            };

            let specialist_id = sess.user_id().to_string();

            let args = AddSpecialistPatientArgs {
                specialist_id,
                patient_email,
            };

            facade
                .add_specialist_patient(args)
                .await
                .map(|_| {
                    state.set(AsyncState::Ready(()));
                })
                .map_err(|e| {
                    state.set(AsyncState::Error(e.clone()));
                    e
                })
        }
    });

    UseAddSpecialistPatient { action, state }
}
