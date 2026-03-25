use dioxus::prelude::*;

use crate::{hooks::app_context::use_app_context, hooks::AsyncState};
use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::list_workout_library::{ListWorkoutLibraryArgs, WorkoutLibraryItem};

#[derive(Clone)]
pub struct UseWorkoutLibrary {
    pub state: Signal<AsyncState<Vec<WorkoutLibraryItem>>>,
    pub resource: Resource<Result<Vec<WorkoutLibraryItem>>>,
}

pub fn use_workout_library(filter: Signal<String>) -> UseWorkoutLibrary {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<Vec<WorkoutLibraryItem>>::Loading);

    let facade_for_resource = facade.clone();

    let resource = use_resource(move || {
        let filter_val = filter();

        let facade = facade_for_resource.clone();
        async move {
            facade
                .list_workout_library(ListWorkoutLibraryArgs {
                    name_filter: Some(filter_val).filter(|s| !s.is_empty()),
                })
                .await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UseWorkoutLibrary { state, resource }
}
