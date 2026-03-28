use std::sync::Arc;

use axum::routing::{get, patch, post};
use axum::Router;

use crate::handlers::specialists;
use crate::router::paths::specialists_path;
use crate::state::AppState;

pub fn specialists_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            &specialists_path(),
            Router::new()
                .route("/login", post(specialists::login_specialist))
                .route(
                    "/add-specialist-patient",
                    post(specialists::add_specialist_patient),
                )
                .route(
                    "/add-exercise-to-workout",
                    post(specialists::add_exercise_to_workout),
                )
                .route(
                    "/assign-program-to-patient",
                    post(specialists::assign_program_to_patient),
                )
                .route("/create-exercise", post(specialists::create_exercise))
                .route("/create-program", post(specialists::create_program))
                .route(
                    "/create-program-schedule-item",
                    post(specialists::create_program_schedule_item),
                )
                .route("/create-workout", post(specialists::create_workout))
                .route(
                    "/delete-program-schedule-item",
                    post(specialists::delete_program_schedule_item),
                )
                .route("/delete-workout", post(specialists::delete_workout))
                .route(
                    "/get-specialist-patients-with-profiles",
                    get(specialists::get_specialist_patients_with_profiles),
                )
                .route(
                    "/specialist-programs-data",
                    get(specialists::specialist_programs_data),
                )
                .route(
                    "/list-exercise-library",
                    get(specialists::list_exercise_library),
                )
                .route(
                    "/list-program-schedule",
                    get(specialists::list_program_schedule),
                )
                .route(
                    "/list-unassigned-patients",
                    get(specialists::list_unassigned_patients),
                )
                .route(
                    "/list-workout-library",
                    get(specialists::list_workout_library),
                )
                .route("/p  atient-progress", get(specialists::patient_progress))
                .route(
                    "/remove-exercise-from-workout",
                    post(specialists::remove_exercise_from_workout),
                )
                .route("/restore-exercise", post(specialists::restore_exercise))
                .route(
                    "/soft-delete-exercise",
                    post(specialists::soft_delete_exercise),
                )
                .route("/update-e   xercise", patch(specialists::update_exercise))
                .route("/update-workout", patch(specialists::update_workout))
                .route(
                    "/update-workout-exercise",
                    patch(specialists::update_workout_exercise),
                )
                .route(
                    "/workout-editor-data",
                    get(specialists::workout_editor_data),
                ),
        )
        .with_state(state)
}
