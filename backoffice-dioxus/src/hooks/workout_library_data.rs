use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::error::Result;
use application::ports::backoffice_api::{ListWorkoutLibraryArgs, ListWorkoutLibraryResult};

#[derive(Clone)]
pub struct UseWorkoutLibraryData {
    pub resource: Resource<Result<ListWorkoutLibraryResult>>,
}

pub fn use_workout_library_data() -> UseWorkoutLibraryData {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();
        async move {
            let args = ListWorkoutLibraryArgs { name_filter: None };
            facade.list_workout_library(args).await
        }
    });

    UseWorkoutLibraryData { resource }
}
