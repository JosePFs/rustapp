use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::delete_workout::DeleteWorkoutArgs;

#[derive(Clone)]
pub struct UseDeleteWorkout {
    pub action: Action<((String,),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_delete_workout() -> UseDeleteWorkout {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(move |(workout_id,): (String,)| {
        let facade = facade_for_action.clone();
        let mut state = state.clone();

        state.set(AsyncState::Loading);

        async move {
            let args = DeleteWorkoutArgs { workout_id };

            facade
                .delete_workout(args)
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

    UseDeleteWorkout { action, state }
}
