use dioxus::prelude::*;

use crate::app_context::UpdateExerciseUseCaseType;
use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::update_exercise::UpdateExerciseArgs;
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseUpdateExercise {
    pub action: Action<((String, String, String, String),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_update_exercise() -> UseUpdateExercise {
    let app_context = use_app_context();
    let use_case = app_context.use_case::<UpdateExerciseUseCaseType>();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(exercise_id, name, description, video_url): (
            String,
            String,
            String,
            String,
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

                let args = UpdateExerciseArgs {
                    token,
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

    UseUpdateExercise { action, state }
}
