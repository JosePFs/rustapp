use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::create_exercise::CreateExerciseArgs;
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseCreateExercise {
    pub action: Action<((String, String, i32, Option<String>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_exercise() -> UseCreateExercise {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(name, description, order_index, video_url): (
            String,
            String,
            i32,
            Option<String>,
        )| {
            let facade = facade_for_action.clone();
            let session_signal = session_signal_for_action.clone();
            let mut state = state.clone();

            state.set(AsyncState::Loading);

            async move {
                let sess_opt = session_signal.read().clone();
                let Some(sess) = sess_opt else {
                    state.set(AsyncState::Error(DomainError::SessionNotFound));
                    return Err(DomainError::SessionNotFound);
                };

                let specialist_id = sess.user_id().to_string();

                let args = CreateExerciseArgs {
                    specialist_id,
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
