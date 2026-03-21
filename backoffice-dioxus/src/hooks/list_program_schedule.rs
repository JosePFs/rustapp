use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::ports::BackofficeApi;
use application::use_cases::list_program_schedule::{ListProgramScheduleArgs, ProgramScheduleData};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseProgramScheduleData {
    pub resource: Resource<Result<ProgramScheduleData, domain::error::DomainError>>,
}

pub fn use_program_schedule_data(program_id: String) -> UseProgramScheduleData {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let session_signal = app_context.session();

    let facade_clone = facade.clone();
    let session_clone = session_signal.clone();
    let program_id_clone = program_id.clone();

    let resource = use_resource(move || {
        let facade = facade_clone.clone();
        let session = session_clone.clone();
        let program_id = program_id_clone.clone();
        async move {
            let sess_opt = session.read().clone();
            let Some(sess) = sess_opt else {
                return Err(DomainError::SessionNotFound);
            };

            let args = ListProgramScheduleArgs {
                token: sess.access_token().to_string(),
                program_id,
            };

            facade.list_program_schedule(args).await
        }
    });

    UseProgramScheduleData { resource }
}
