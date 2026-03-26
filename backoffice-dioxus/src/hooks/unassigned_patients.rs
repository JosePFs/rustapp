use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::list_unassigned_patients::{
    UnassignedPatientsArgs, UnassignedPatientsResult,
};

#[derive(Clone)]
pub struct UseUnassignedPatients {
    pub state: Signal<AsyncState<UnassignedPatientsResult>>,
    pub resource: Resource<Result<UnassignedPatientsResult>>,
}

pub fn use_unassigned_patients() -> UseUnassignedPatients {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<UnassignedPatientsResult>::Idle);

    let facade_for_resource = facade.clone();

    let resource = use_resource(move || {
        let facade = facade_for_resource.clone();

        async move {
            facade
                .list_unassigned_patients(UnassignedPatientsArgs {})
                .await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UseUnassignedPatients { state, resource }
}
