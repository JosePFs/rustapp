use std::collections::HashMap;

use dioxus::prelude::*;

use crate::hooks::{app_context::use_app_context, AsyncState};
use application::use_cases::update_patient_workout_feedback::{
    UpdatePatientWorkoutFeedbackArgs, UpdatePatientWorkoutFeedbackUseCase,
};
use domain::error::DomainError;

#[derive(Clone)]
pub struct UseSubmitWorkoutFeedback {
    pub action: Action<((),), ()>,
    pub state: Signal<AsyncState<()>>,
}

pub fn use_submit_workout_feedback(
    patient_program_id: String,
    day_index: i32,
    session_date: Signal<String>,
    exercise_feedback: Signal<HashMap<String, (i32, i32, String)>>,
    feedback_completed: bool,
) -> UseSubmitWorkoutFeedback {
    let app_context = use_app_context();
    let submit_use_case: std::sync::Arc<UpdatePatientWorkoutFeedbackUseCase<_>> =
        app_context.submit_patient_workout_feedback_use_case();
    let session_signal = app_context.session();
    let state = use_signal(|| AsyncState::Idle);

    let submit_use_case_for_action = submit_use_case.clone();
    let session_signal_for_action = session_signal.clone();

    let action = use_action(move |(): ()| {
        let submit_use_case = submit_use_case_for_action.clone();
        let session_signal = session_signal_for_action.clone();
        let patient_program_id = patient_program_id.clone();
        let session_date = session_date.clone();
        let exercise_feedback = exercise_feedback.clone();
        let mut state = state.clone();

        state.set(AsyncState::Loading);

        async move {
            let sess_opt = session_signal.read().clone();
            let Some(sess) = sess_opt else {
                state.set(AsyncState::Error(DomainError::SessionNotFound));
                return Err(DomainError::SessionNotFound);
            };

            let token = sess.access_token().to_string();
            let date_str = session_date();
            let fb_map = exercise_feedback();

            let args = UpdatePatientWorkoutFeedbackArgs {
                token,
                patient_program_id,
                day_index,
                session_date: date_str,
                feedback_completed,
                feedback_map: fb_map,
            };

            submit_use_case
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

    UseSubmitWorkoutFeedback { action, state }
}
