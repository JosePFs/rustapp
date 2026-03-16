use dioxus::prelude::*;

use crate::application::use_cases::patient_workout_session::{
    PatientWorkoutSession, PatientWorkoutSessionArgs, PatientWorkoutSessionUseCase,
};
use crate::domain::error::{DomainError, Result};
use crate::infrastructure::ui::hooks::app_context::use_app_context;

pub fn use_workout_day_detail(
    patient_program_id: String,
    day_index: i32,
) -> Resource<Result<PatientWorkoutSession>> {
    let app_ctx = use_app_context();
    let patient_workout_use_case: std::sync::Arc<PatientWorkoutSessionUseCase<_>> =
        app_ctx.patient_workout_session_use_case();

    use_resource(move || {
        let app_session = app_ctx.session().read().clone();
        let patient_program_id = patient_program_id.clone();
        let patient_workout_use_case = patient_workout_use_case.clone();

        async move {
            let sess = app_session.ok_or(DomainError::SessionNotFound)?;
            let token = sess.access_token().to_string();

            patient_workout_use_case
                .execute(PatientWorkoutSessionArgs {
                    token,
                    patient_program_id,
                    day_index,
                })
                .await
        }
    })
}
