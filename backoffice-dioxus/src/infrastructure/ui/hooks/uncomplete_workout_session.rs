use dioxus::prelude::*;

use crate::application::use_cases::uncomplete_patient_workout_session::{
    UncompletePatientWorkoutSessionArgs, UncompletePatientWorkoutSessionUseCase,
};
use crate::domain::error::DomainError;
use crate::infrastructure::ui::hooks::{app_context::use_app_context, AsyncState};

#[derive(Clone)]
pub struct UseUncompleteWorkoutSession {
    pub action: Action<(String,), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_uncomplete_workout_session() -> UseUncompleteWorkoutSession {
    let app_context = use_app_context();
    let use_case: std::sync::Arc<UncompletePatientWorkoutSessionUseCase<_>> =
        app_context.uncomplete_patient_workout_session_use_case();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let use_case_for_action = use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(move |session_id: String| {
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

            let args = UncompletePatientWorkoutSessionArgs { token, session_id };

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

    UseUncompleteWorkoutSession { action, state }
}
