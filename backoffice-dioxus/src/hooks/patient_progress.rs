use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::patient_progress::{PatientProgressArgs, PatientProgressResult};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UsePatientProgress {
    pub state: Signal<AsyncState<PatientProgressResult>>,
}

pub fn use_patient_progress(patient_id: String) -> UsePatientProgress {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<PatientProgressResult>::Loading);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let facade = facade.clone();
        let patient_id = patient_id.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };

            let token = session.access_token().to_string();

            facade
                .patient_progress(PatientProgressArgs { token, patient_id })
                .await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),

        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),

        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UsePatientProgress { state }
}
