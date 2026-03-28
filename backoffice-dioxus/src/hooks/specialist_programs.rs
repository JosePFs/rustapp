use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::SpecialistProgramsDataResult;

#[derive(Clone)]
pub struct UseSpecialistPrograms {
    pub state: Signal<AsyncState<SpecialistProgramsDataResult>>,
    pub resource: Resource<SpecialistProgramsDataResult>,
}

pub fn use_specialist_programs() -> UseSpecialistPrograms {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<SpecialistProgramsDataResult>::Idle);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();
        async move {
            state.set(AsyncState::Loading);

            match facade.specialist_programs_data().await {
                Ok(result) => {
                    state.set(AsyncState::Ready(result.clone()));
                    result
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    SpecialistProgramsDataResult {
                        links: vec![],
                        profiles: vec![],
                        programs: vec![],
                        assignments: vec![],
                    }
                }
            }
        }
    });

    UseSpecialistPrograms { state, resource }
}
