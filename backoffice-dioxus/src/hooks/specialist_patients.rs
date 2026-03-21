use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::get_specialist_patients_with_profiles::{
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
};
use domain::error::{DomainError, Result};

#[derive(Clone)]
pub struct UseSpecialistPatients {
    pub state: Signal<AsyncState<GetSpecialistPatientsWithProfilesResult>>,
    pub resource: Resource<Result<GetSpecialistPatientsWithProfilesResult>>,
}

pub fn use_specialist_patients() -> UseSpecialistPatients {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<GetSpecialistPatientsWithProfilesResult>::Loading);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let facade = facade.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };
            let token = session.access_token().to_string();
            facade
                .get_specialist_patients_with_profiles(GetSpecialistPatientsWithProfilesArgs {
                    token,
                })
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
