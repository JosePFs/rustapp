use dioxus::prelude::*;

use crate::app_context::ListExerciseLibraryUseCaseType;
use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::list_exercise_library::ListExerciseLibraryArgs;
use domain::{
    entities::Exercise,
    error::{DomainError, Result},
};

#[derive(Clone)]
pub struct UseExerciseLibrary {
    pub state: Signal<AsyncState<Vec<Exercise>>>,
    pub resource: Resource<Result<Vec<Exercise>>>,
}

pub fn use_exercise_library(filter: Signal<String>) -> UseExerciseLibrary {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let use_case = app_context.use_case::<ListExerciseLibraryUseCaseType>();
    let mut state = use_signal(|| AsyncState::<Vec<Exercise>>::Loading);

    let use_case = use_case.clone();
    let resource = use_resource(move || {
        let filter_val = filter();
        let maybe_session_ref = app_session.read().clone();
        let use_case = use_case.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };
            let token = session.access_token().to_string();
            let specialist_id = session.user_id().to_string();
            use_case
                .execute(ListExerciseLibraryArgs {
                    token,
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
