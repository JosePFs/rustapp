use dioxus::prelude::*;

use crate::application::use_cases::get_specialist_patients_with_profiles::{
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
};
use crate::domain::error::DomainError;
use crate::infrastructure::ui::hooks::app_context::use_app_context;
use crate::infrastructure::ui::hooks::AsyncState;

#[derive(Clone)]
pub struct UseSpecialistPatients {
    pub state: Signal<AsyncState<GetSpecialistPatientsWithProfilesResult>>,
    pub resource: Resource<Result<GetSpecialistPatientsWithProfilesResult, DomainError>>,
}

pub fn use_specialist_patients() -> UseSpecialistPatients {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let use_case = app_context.get_specialist_patients_with_profiles_use_case();
    let mut state = use_signal(|| AsyncState::<GetSpecialistPatientsWithProfilesResult>::Loading);

    let use_case = use_case.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let use_case = use_case.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };
            let token = session.access_token().to_string();
            use_case
                .execute(GetSpecialistPatientsWithProfilesArgs { token })
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
