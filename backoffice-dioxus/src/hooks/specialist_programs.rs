use dioxus::prelude::*;

use crate::app_context::SpecialistProgramsDataUseCaseType;
use application::use_cases::specialist_programs_data::{
    SpecialistProgramsDataArgs, SpecialistProgramsDataResult,
};
use domain::error::{DomainError, Result};

use crate::hooks::{app_context::use_app_context, AsyncState};

#[derive(Clone)]
pub struct UseSpecialistPrograms {
    pub state: Signal<AsyncState<SpecialistProgramsDataResult>>,
    pub resource: Resource<Result<SpecialistProgramsDataResult>>,
}

pub fn use_specialist_programs() -> UseSpecialistPrograms {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let use_case = app_context.use_case::<SpecialistProgramsDataUseCaseType>();
    let mut state = use_signal(|| AsyncState::<SpecialistProgramsDataResult>::Loading);

    let use_case = use_case.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let use_case = use_case.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };
            let token = session.access_token().to_string();
            use_case.execute(SpecialistProgramsDataArgs { token }).await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UseSpecialistPrograms { state, resource }
}
