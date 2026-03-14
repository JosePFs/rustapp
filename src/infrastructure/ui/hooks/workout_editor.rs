use dioxus::prelude::*;

use crate::application::use_cases::workout_editor_data::{
    WorkoutEditorDataArgs, WorkoutEditorDataResult,
};
use crate::domain::error::DomainError;
use crate::infrastructure::ui::hooks::app_context::use_app_context;
use crate::infrastructure::ui::hooks::AsyncState;

#[derive(Clone)]
pub struct UseWorkoutEditor {
    pub state: Signal<AsyncState<WorkoutEditorDataResult>>,
    pub resource: Resource<Result<WorkoutEditorDataResult, DomainError>>,
}

pub fn use_workout_editor(workout_id: String) -> UseWorkoutEditor {
    let app_context = use_app_context();
    let app_session = app_context.session();
    let use_case = app_context.workout_editor_data_use_case();
    let mut state = use_signal(|| AsyncState::<WorkoutEditorDataResult>::Loading);

    let use_case = use_case.clone();
    let resource = use_resource(move || {
        let maybe_session_ref = app_session.read().clone();
        let use_case = use_case.clone();
        let workout_id = workout_id.clone();

        async move {
            let Some(session) = maybe_session_ref.as_ref() else {
                return Err(DomainError::SessionNotFound);
            };
            let token = session.access_token().to_string();
            let specialist_id = session.user_id().to_string();
            use_case
                .execute(WorkoutEditorDataArgs {
                    token,
                    specialist_id,
                    workout_id,
                })
                .await
        }
    });

    use_effect(move || match resource.read().as_ref() {
        None => state.set(AsyncState::Loading),
        Some(Err(e)) => state.set(AsyncState::Error(e.clone())),
        Some(Ok(data)) => state.set(AsyncState::Ready(data.clone())),
    });

    UseWorkoutEditor { state, resource }
}
