use dioxus::prelude::*;

use crate::{hooks::app_context::use_app_context, hooks::AsyncState};
use application::ports::backoffice_api::{WorkoutEditorDataArgs, WorkoutEditorDataResult};

#[derive(Clone)]
pub struct UseWorkoutEditor {
    pub state: Signal<AsyncState<WorkoutEditorDataResult>>,
    pub resource: Resource<WorkoutEditorDataResult>,
}

pub fn use_workout_editor(workout_id: String) -> UseWorkoutEditor {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let mut state = use_signal(|| AsyncState::<WorkoutEditorDataResult>::Idle);

    let facade = facade.clone();
    let resource = use_resource(move || {
        let facade = facade.clone();
        let workout_id = workout_id.clone();

        async move {
            state.set(AsyncState::Loading);

            let args = WorkoutEditorDataArgs { workout_id };
            match facade.workout_editor_data(args).await {
                Ok(result) => {
                    state.set(AsyncState::Ready(result.clone()));
                    result
                }
                Err(e) => {
                    state.set(AsyncState::Error(e.clone()));
                    WorkoutEditorDataResult {
                        workout: None,
                        exercises: vec![],
                        library: vec![],
                    }
                }
            }
        }
    });

    UseWorkoutEditor { state, resource }
}
