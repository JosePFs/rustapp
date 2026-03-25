use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::list_exercise_library::{ExerciseLibraryItem, ListExerciseLibraryArgs};

#[derive(Clone)]
pub struct UseExerciseLibrary {
    pub filter: Signal<String>,
    pub state: Signal<AsyncState<Vec<ExerciseLibraryItem>>>,
    pub resource: Resource<Result<Vec<ExerciseLibraryItem>>>,
}

pub fn use_exercise_library() -> UseExerciseLibrary {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let filter = use_signal(|| String::new());
    let mut state = use_signal(|| AsyncState::<Vec<ExerciseLibraryItem>>::Loading);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let filter_val = filter();
        let facade = facade.clone();

        async move {
            facade
                .list_exercise_library(ListExerciseLibraryArgs {
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

    UseExerciseLibrary {
        filter,
        state,
        resource,
    }
}
