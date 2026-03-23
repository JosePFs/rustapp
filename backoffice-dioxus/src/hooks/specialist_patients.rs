use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::get_specialist_patients_with_profiles::{
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
};
use domain::error::Result;

#[derive(Clone)]
pub struct UseSpecialistPatients {
    pub state: Signal<AsyncState<GetSpecialistPatientsWithProfilesResult>>,
    pub resource: Resource<Result<GetSpecialistPatientsWithProfilesResult>>,
}

pub fn use_specialist_patients() -> UseSpecialistPatients {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<GetSpecialistPatientsWithProfilesResult>::Loading);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();

        async move {
            facade
                .get_specialist_patients_with_profiles(GetSpecialistPatientsWithProfilesArgs {})
                .await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(GetSpecialistPatientsWithProfilesResult {
            links: data.links.clone(),
            profiles: data.profiles.clone(),
        })),
    });

    UseSpecialistPatients { state, resource }
}
