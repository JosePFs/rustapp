use std::sync::Arc;

use axum::{http::HeaderValue, Router};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    router::{
        fallback::fallback, health::health_routes, paths::API_BASE_PATH,
        patients_routes::patients_routes, specialists_routes::specialists_routes,
    },
    state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router<()> {
    let health_routes = health_routes();
    let patients_routes = patients_routes(state.clone());
    let specialists_routes = specialists_routes(state.clone());
    let router = Router::new()
        .merge(patients_routes)
        .merge(specialists_routes);

    let cors_allowed_origins = state
        .clone()
        .config()
        .cors_allowed_origins()
        .split(",")
        .map(|s| HeaderValue::from_str(s).unwrap())
        .collect::<Vec<HeaderValue>>();

    Router::new()
        .merge(health_routes)
        .nest(API_BASE_PATH, router)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list(cors_allowed_origins))
                .allow_methods(Any)
                .allow_headers(Any),
        )
}
