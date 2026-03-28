use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::UpdateWorkoutExerciseArgs;

#[derive(Clone)]
pub struct UseUpdateWorkoutExercise {
    pub action: Action<((String, String, i32, i32, Option<i32>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_update_workout_exercise() -> UseUpdateWorkoutExercise {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(
        move |(workout_id, exercise_id, sets, reps, order_index): (
            String,
            String,
            i32,
            i32,
            Option<i32>,
        )| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let args = UpdateWorkoutExerciseArgs {
                    workout_id,
                    exercise_id,
                    sets,
                    reps,
                    order_index,
                };

                facade
                    .update_workout_exercise(args)
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

    UseUpdateWorkoutExercise { action, state }
}
