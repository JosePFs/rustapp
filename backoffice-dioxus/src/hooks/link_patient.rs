use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::add_specialist_patient::AddSpecialistPatientArgs;

#[derive(Clone)]
pub struct UseLinkPatient {
    pub action: Action<(String,), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_link_patient() -> UseLinkPatient {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(move |patient_email: String| {
        let facade = facade_for_action.clone();
        let mut state = state.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = AddSpecialistPatientArgs { patient_email };

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

    UseLinkPatient { action, state }
}