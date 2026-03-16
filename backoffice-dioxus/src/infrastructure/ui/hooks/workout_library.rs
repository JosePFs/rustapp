use dioxus::prelude::*;

use crate::application::use_cases::list_workout_library::ListWorkoutLibraryArgs;
use crate::domain::entities::Workout;
use crate::domain::error::DomainError;
use crate::infrastructure::ui::hooks::app_context::use_app_context;
use crate::infrastructure::ui::hooks::AsyncState;

#[derive(Clone)]
pub struct UseWorkoutLibrary {
    pub state: Signal<AsyncState<Vec<Workout>>>,
    pub resource: Resource<Result<Vec<Workout>, DomainError>>,
}

pub fn use_workout_library(filter: Signal<String>) -> UseWorkoutLibrary {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let use_case = app_context.list_workout_library_use_case();
    let mut state = use_signal(|| AsyncState::<Vec<Workout>>::Loading);

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
                .execute(ListWorkoutLibraryArgs {
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

    UseWorkoutLibrary { state, resource }
}
