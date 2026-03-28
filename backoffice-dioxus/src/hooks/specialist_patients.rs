use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::{GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult};

#[derive(Clone)]
pub struct SpecialistPatientsData {
    pub links: Vec<application::ports::backoffice_api::SpecialistPatientLink>,
    pub profiles: Vec<application::ports::backoffice_api::PatientProfileSummary>,
    pub patients: Vec<GetSpecialistPatientsWithProfilesResult>,
}

impl Default for SpecialistPatientsData {
    fn default() -> Self {
        Self {
            links: vec![],
            profiles: vec![],
            patients: vec![],
        }
    }
}

#[derive(Clone)]
pub struct UseSpecialistPatients {
    pub state: Signal<AsyncState<SpecialistPatientsData>>,
    pub resource: Resource<SpecialistPatientsData>,
}

pub fn use_specialist_patients() -> UseSpecialistPatients {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<SpecialistPatientsData>::Idle);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = GetSpecialistPatientsWithProfilesArgs {};
            match facade.get_specialist_patients_with_profiles(args).await {
                Ok(result) => {
                    let data = SpecialistPatientsData {
                        links: result.links.clone(),
                        profiles: result.profiles.clone(),
                        patients: vec![result],
                    };
                    state.set(AsyncState::Ready(data.clone()));
                    data
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    SpecialistPatientsData::default()
                }
            }
        }
    });

    UseSpecialistPatients { state, resource }
}
