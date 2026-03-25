use dioxus::prelude::*;

use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::specialist_programs_data::SpecialistProgramsDataResult;

use crate::hooks::{app_context::use_app_context, AsyncState};

#[derive(Clone)]
pub struct UseSpecialistPrograms {
    pub state: Signal<AsyncState<SpecialistProgramsDataResult>>,
    pub resource: Resource<Result<SpecialistProgramsDataResult>>,
}

pub fn use_specialist_programs() -> UseSpecialistPrograms {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<SpecialistProgramsDataResult>::Loading);

    let facade_for_resource = facade.clone();

    let resource = use_resource(move || {
        let facade = facade_for_resource.clone();
        async move { facade.specialist_programs_data().await }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UseSpecialistPrograms { state, resource }
}
