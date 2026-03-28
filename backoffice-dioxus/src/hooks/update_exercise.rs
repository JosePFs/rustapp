use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::backoffice_api::UpdateExerciseArgs;

#[derive(Clone)]
pub struct UseUpdateExercise {
    pub action: Action<((String, String, String, String),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_update_exercise() -> UseUpdateExercise {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();

    let action = use_action(
        move |(exercise_id, name, description, video_url): (String, String, String, String)| {
            let facade = facade_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let args = UpdateExerciseArgs {
                    exercise_id,
                    name: if name.is_empty() { None } else { Some(name) },
                    description: if description.is_empty() {
                        None
                    } else {
                        Some(description)
                    },
                    order_index: None,
                    video_url: if video_url.is_empty() {
                        None
                    } else {
                        Some(video_url)
                    },
                };

                facade
                    .update_exercise(args)
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

    UseUpdateExercise { action, state }
}
