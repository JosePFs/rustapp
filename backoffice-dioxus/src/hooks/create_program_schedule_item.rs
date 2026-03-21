use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::ports::BackofficeApi;
use application::use_cases::create_program_schedule_item::CreateProgramScheduleItemArgs;
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseCreateProgramScheduleItem {
    pub action: Action<((String, i32, Option<String>, i32),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_create_program_schedule_item() -> UseCreateProgramScheduleItem {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let facade_for_action = facade.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(
        move |(program_id, order_index, workout_id, days_count): (
            String,
            i32,
            Option<String>,
            i32,
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

                let token = sess.access_token().to_string();

                let args = CreateProgramScheduleItemArgs {
                    token,
                    program_id,
                    order_index,
                    workout_id,
                    days_count,
                };

                facade
                    .create_program_schedule_item(args)
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

    UseCreateProgramScheduleItem { action, state }
}
