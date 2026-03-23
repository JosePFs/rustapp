use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::list_exercise_library::{ExerciseLibraryItem, ListExerciseLibraryArgs};
use domain::error::{DomainError, Result};

#[derive(Clone)]
pub struct UseExerciseLibrary {
    pub state: Signal<AsyncState<Vec<ExerciseLibraryItem>>>,
    pub resource: Resource<Result<Vec<ExerciseLibraryItem>>>,
}

pub fn use_exercise_library(filter: Signal<String>) -> UseExerciseLibrary {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<Vec<ExerciseLibraryItem>>::Loading);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let filter_val = filter();
        let maybe_session_ref = app_session.read().clone();
        let facade = facade.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };

            let specialist_id = session.user_id().to_string();

            facade
                .list_exercise_library(ListExerciseLibraryArgs {
                    specialist_id,
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

    UseExerciseLibrary { state, resource }
}
