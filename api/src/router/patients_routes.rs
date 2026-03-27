use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handlers::{
    get_programs, login, mark_day_as_completed, mark_day_as_uncompleted, refresh_session,
};
use crate::{router::paths::patients_path, state::AppState};

pub fn patients_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route(&patients_path(Some("login".to_string())), post(login))
        .route(
            &patients_path(Some("refresh-session".to_string())),
            post(refresh_session),
        )
        .route(
            &patients_path(Some("get-programs".to_string())),
            get(get_programs),
        )
        .route(
            &patients_path(Some("mark-day-as-completed".to_string())),
            post(mark_day_as_completed),
        )
        .route(
            &patients_path(Some("mark-day-as-uncompleted".to_string())),
            post(mark_day_as_uncompleted),
        )
        .with_state(state)
}
