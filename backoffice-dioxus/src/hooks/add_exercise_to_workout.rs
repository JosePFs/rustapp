use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::AddExerciseToWorkoutArgs;

#[derive(Clone)]
pub struct UseAddExerciseToWorkout {
    pub action: Action<((String, String, i32, i32, i32),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_add_exercise_to_workout() -> UseAddExerciseToWorkout {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(
        move |(workout_id, exercise_id, order_index, sets, reps): (
            String,
            String,
            i32,
            i32,
            i32,
        )| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let args = AddExerciseToWorkoutArgs {
                    workout_id,
                    exercise_id,
                    order_index,
                    sets,
                    reps,
                };

                facade
                    .add_exercise_to_workout(args)
                    .await
                    .map(|_| {
                        state.set(AsyncState::Ready(()));
                    })
                    .map_err(|e| {
                        state.set(AsyncState::Error(e.clone()));
                        e
                    })
            }
        },
    );

    UseAddExerciseToWorkout { action, state }
}
