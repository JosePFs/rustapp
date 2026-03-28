use dioxus::prelude::*;

use crate::{hooks::app_context::use_app_context, hooks::AsyncState};
use application::ports::backoffice_api::{ListWorkoutLibraryArgs, ListWorkoutLibraryResult, WorkoutLibraryItem};

#[derive(Clone)]
pub struct UseWorkoutLibrary {
    pub state: Signal<AsyncState<Vec<WorkoutLibraryItem>>>,
    pub resource: Resource<Vec<WorkoutLibraryItem>>,
}

pub fn use_workout_library(filter: Signal<String>) -> UseWorkoutLibrary {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<Vec<WorkoutLibraryItem>>::Idle);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let filter_val = filter();
        let facade = facade.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = ListWorkoutLibraryArgs {
                name_filter: if filter_val.is_empty() {
                    None
                } else {
                    Some(filter_val)
                },
            };
            match facade.list_workout_library(args).await {
                Ok(result) => {
                    state.set(AsyncState::Ready(result.items.clone()));
                    result.items
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    vec![]
                }
            }
        }
    });

    UseWorkoutLibrary { state, resource }
}
