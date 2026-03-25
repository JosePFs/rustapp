use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::create_workout::CreateWorkoutArgs;

#[derive(Clone)]
pub struct UseCreateWorkout {
    pub action: Action<(String, String), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_workout() -> UseCreateWorkout {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action: Action<(String, String), ()> = use_action(move |name: String, description: String| {
        let facade = facade_for_action.clone();
        let mut state = state.clone();

        state.set(AsyncState::Loading);

        async move {
            let args = CreateWorkoutArgs {
                name,
                description: if description.is_empty() {
                    None
                } else {
                    Some(description)
                },
            };

            facade
                .create_workout(args)
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

    UseCreateWorkout { action, state }
}