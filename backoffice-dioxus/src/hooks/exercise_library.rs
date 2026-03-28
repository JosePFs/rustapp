use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::{
    ExerciseLibraryItem, ListExerciseLibraryArgs, ListExerciseLibraryResult,
};

#[derive(Clone)]
pub struct UseExerciseLibrary {
    pub filter: Signal<String>,
    pub state: Signal<AsyncState<Vec<ExerciseLibraryItem>>>,
    pub resource: Resource<Vec<ExerciseLibraryItem>>,
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
            let args = ListExerciseLibraryArgs {
                name_filter: if filter_val.is_empty() {
                    None
                } else {
                    Some(filter_val)
                },
            };
            match facade.list_exercise_library(args).await {
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

    UseExerciseLibrary {
        filter,
        state,
        resource,
    }
}
