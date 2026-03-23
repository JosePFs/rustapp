use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::patient_progress::{PatientProgressArgs, PatientProgressResult};

#[derive(Clone)]
pub struct UsePatientProgress {
    pub state: Signal<AsyncState<PatientProgressResult>>,
}

pub fn use_patient_progress(patient_id: String) -> UsePatientProgress {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<PatientProgressResult>::Loading);

    let facade = facade.clone();

    let resource = use_resource(move || {
        let facade = facade.clone();
        let patient_id = patient_id.clone();

        async move {
            facade
                .patient_progress(PatientProgressArgs { patient_id })
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
