use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::list_workout_library::WorkoutLibraryItem;

#[derive(Clone)]
pub struct UseWorkoutLibraryData {
    pub resource: Resource<Result<Vec<WorkoutLibraryItem>>>,
}

pub fn use_workout_library_data() -> UseWorkoutLibraryData {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();

    let facade_for_resource = facade.clone();

    let resource = use_resource(move || {
        let facade = facade_for_resource.clone();
        async move { facade.list_workout_library(Default::default()).await }
    });

    UseWorkoutLibraryData { resource }
}
