use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::patients;
use crate::router::paths::patients_path;
use crate::state::AppState;

pub fn patients_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            &patients_path(),
            Router::new()
                .route("/login", post(patients::login_patient))
                .route("/refresh-session", post(patients::refresh_session))
                .route("/get-programs", get(patients::get_programs))
                .route(
                    "/mark-day-as-completed",
                    post(patients::mark_day_as_completed),
                )
                .route(
                    "/mark-day-as-uncompleted",
                    post(patients::mark_day_as_uncompleted),
                ),
        )
        .with_state(state)
}
