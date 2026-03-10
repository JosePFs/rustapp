use dioxus::prelude::*;

use crate::application::use_cases::get_patient_programs::{
    GetPatientProgramsUseCaseArgs, GetPatientProgramsUseCaseResult,
};
use crate::domain::error::DomainError;
use crate::infrastructure::ui::hooks::app_context::use_app_context;
use crate::infrastructure::ui::hooks::AsyncState;

#[derive(Clone)]
pub struct UsePatientPrograms {
    pub state: Signal<AsyncState<GetPatientProgramsUseCaseResult>>,
}

pub fn use_patient_programs() -> UsePatientPrograms {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let get_patient_programs_use_case = app_context.get_patient_programs_use_case();
    let mut state = use_signal(|| AsyncState::<GetPatientProgramsUseCaseResult>::Loading);

    let get_patient_programs_use_case = get_patient_programs_use_case.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let get_patient_programs_use_case = get_patient_programs_use_case.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };

            let token = session.access_token().to_string();

            get_patient_programs_use_case
                .execute(GetPatientProgramsUseCaseArgs { token })
                .await
                .map(|patient_programs| patient_programs)
                .map_err(|e| e)
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),

        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),

        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UsePatientPrograms { state }
}
