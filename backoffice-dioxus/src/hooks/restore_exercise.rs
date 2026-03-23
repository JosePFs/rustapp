use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::restore_exercise::RestoreExerciseArgs;

#[derive(Clone)]
pub struct UseRestoreExercise {
    pub action: Action<((String,),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_restore_exercise() -> UseRestoreExercise {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(move |(exercise_id,): (String,)| {
        let facade = facade_for_action.clone();
        let mut state = state.clone();

        state.set(AsyncState::Loading);

        async move {
            let args = RestoreExerciseArgs { exercise_id };

            facade
                .restore_exercise(args)
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

    UseRestoreExercise { action, state }
}
