use dioxus::prelude::*;

use crate::app_context::ListProgramScheduleUseCaseType;
use crate::hooks::app_context::use_app_context;
use application::use_cases::list_program_schedule::{ListProgramScheduleArgs, ListProgramScheduleUseCase, ProgramScheduleData};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseProgramScheduleData {
    pub resource: Resource<Result<ProgramScheduleData, domain::error::DomainError>>,
}

pub fn use_program_schedule_data(program_id: String) -> UseProgramScheduleData {
    let app_context = use_app_context();
    let use_case = app_context.use_case::<ListProgramScheduleUseCaseType>();
    let session_signal = app_context.session();

    let use_case_clone = use_case.clone();
    let session_clone = session_signal.clone();
    let program_id_clone = program_id.clone();

    let resource = use_resource(move || {
        let use_case = use_case_clone.clone();
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

            use_case.execute(args).await
        }
    });

    UseProgramScheduleData { resource }
}
