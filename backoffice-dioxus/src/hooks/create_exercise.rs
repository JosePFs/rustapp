use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::CreateExerciseArgs;

#[derive(Clone)]
pub struct UseCreateExercise {
    pub action: Action<((String, String, i32, Option<String>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_exercise() -> UseCreateExercise {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(
        move |(name, description, order_index, video_url): (
            String,
            String,
            i32,
            Option<String>,
        )| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            async move {
                state.set(AsyncState::Loading);

                let args = CreateExerciseArgs {
                    name,
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description)
                    },
                    order_index,
                    video_url,
                };

                facade
                    .create_exercise(args)
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

    UseCreateExercise { action, state }
}
