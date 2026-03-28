use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::{ListUnassignedPatientsArgs, ListUnassignedPatientsResult};

#[derive(Clone)]
pub struct UseUnassignedPatients {
    pub state: Signal<AsyncState<ListUnassignedPatientsResult>>,
    pub resource: Resource<ListUnassignedPatientsResult>,
}

pub fn use_unassigned_patients() -> UseUnassignedPatients {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<ListUnassignedPatientsResult>::Idle);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = ListUnassignedPatientsArgs {};
            match facade.list_unassigned_patients(args).await {
                Ok(result) => {
                    state.set(AsyncState::Ready(result.clone()));
                    result
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    ListUnassignedPatientsResult { patients: vec![] }
                }
            }
        }
    });

    UseUnassignedPatients { state, resource }
}
