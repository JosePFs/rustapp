use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::remove_exercise_from_workout::{
    RemoveExerciseFromWorkoutArgs, RemoveExerciseFromWorkoutUseCase,
};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseRemoveExerciseFromWorkout {
    pub action: Action<((String, String),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_remove_exercise_from_workout() -> UseRemoveExerciseFromWorkout {
    let app_context = use_app_context();
    let use_case: std::sync::Arc<
        RemoveExerciseFromWorkoutUseCase<infrastructure::supabase::api::Api>,
    > = app_context
        .remove_exercise_from_workout_use_case();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(move |(workout_id, exercise_id): (String, String)| {
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

            let args = RemoveExerciseFromWorkoutArgs {
                token,
                workout_id,
                exercise_id,
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
    });

    UseRemoveExerciseFromWorkout { action, state }
}
