use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::error::Result;
use application::ports::backoffice_api::{ListProgramScheduleArgs, ListProgramScheduleResult};

#[derive(Clone)]
pub struct UseProgramScheduleData {
    pub resource: Resource<Result<ListProgramScheduleResult>>,
}

pub fn use_program_schedule_data(program_id: String) -> UseProgramScheduleData {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();

    let facade = facade.clone();
    let program_id_clone = program_id.clone();

    let resource = use_resource(move || {
        let facade = facade.clone();
        let program_id = program_id_clone.clone();

        async move {
            let args = ListProgramScheduleArgs { program_id };
            facade.list_program_schedule(args).await
        }
    });

    UseProgramScheduleData { resource }
}
