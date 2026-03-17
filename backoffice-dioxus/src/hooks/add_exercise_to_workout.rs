use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::add_exercise_to_workout::{
    AddExerciseToWorkoutArgs, AddExerciseToWorkoutUseCase,
};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseAddExerciseToWorkout {
    pub action: Action<((String, String, i32, i32, i32),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_add_exercise_to_workout() -> UseAddExerciseToWorkout {
    let app_context = use_app_context();
    let use_case: std::sync::Arc<
        AddExerciseToWorkoutUseCase<infrastructure::supabase::api::Api>,
    > = app_context.add_exercise_to_workout_use_case();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(workout_id, exercise_id, order_index, sets, reps): (
            String,
            String,
            i32,
            i32,
            i32,
        )| {
            let use_case = use_case_for_action.clone();
            let session_signal = session_signal_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let sess_opt = session_signal.read().clone();
                let Some(sess) = sess_opt else {
                    state.set(AsyncState::Error(DomainError::SessionNotFound));
                    return Err(DomainError::SessionNotFound);
                };

                let token = sess.access_token().to_string();

                let args = AddExerciseToWorkoutArgs {
                    token,
                    workout_id,
                    exercise_id,
                    order_index,
                    sets,
                    reps,
                };

                use_case
                    .execute(args)
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
