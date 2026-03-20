use dioxus::prelude::*;

use crate::app_context::DeleteProgramScheduleItemUseCaseType;
use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::delete_program_schedule_item::{
    DeleteProgramScheduleItemArgs, DeleteProgramScheduleItemUseCase,
};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseDeleteProgramScheduleItem {
    pub action: Action<((String,),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_delete_program_schedule_item() -> UseDeleteProgramScheduleItem {
    let app_context = use_app_context();
    let use_case = app_context.use_case::<DeleteProgramScheduleItemUseCaseType>();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(move |(schedule_item_id,): (String,)| {
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

            let args = DeleteProgramScheduleItemArgs {
                token,
                schedule_item_id,
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

    UseDeleteProgramScheduleItem { action, state }
}
