use std::sync::Arc;

use axum::{http::HeaderValue, Router};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    router::paths::api_path,
    router::{fallback::fallback, health::health_routes, patients_routes::patients_routes},
    state::AppState,
};

pub fn routes(state: Arc<AppState>) -> Router<()> {
    let health_routes = health_routes();
    let patients_routes = patients_routes(state.clone());

    Router::new()
        .merge(health_routes)
        .nest(&api_path(), patients_routes)
        .fallback(fallback)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::list(
                    state
                        .config()
                        .cors_allowed_origins()
                        .split(",")
                        .map(|s| HeaderValue::from_str(s).unwrap())
                        .collect::<Vec<HeaderValue>>(),
                ))
                .allow_methods(Any)
                .allow_headers(Any),
        )
}
