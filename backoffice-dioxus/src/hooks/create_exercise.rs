use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::create_exercise::{CreateExerciseArgs, CreateExerciseUseCase};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseCreateExercise {
    pub action: Action<((String, String, i32, Option<String>),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_exercise() -> UseCreateExercise {
    let app_context = use_app_context();
    let use_case: std::sync::Arc<
        CreateExerciseUseCase<infrastructure::supabase::api::Api>,
    > = app_context.create_exercise_use_case();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(name, description, order_index, video_url): (
            String,
            String,
            i32,
            Option<String>,
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
                let specialist_id = sess.user_id().to_string();

                let args = CreateExerciseArgs {
                    token,
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

    UseCreateExercise { action, state }
}
