use std::{net::SocketAddr, sync::Arc};

use tokio::net::TcpListener;

use api::{
    error::{Error, Result},
    router::router::routes,
    state::AppState,
    trace,
};

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::builder().build());

    let app_router = routes(state.clone());

    let host = state.config().host();
    let port = state.config().port();
    let addr = SocketAddr::new(host, port);
    let listener = TcpListener::bind(addr).await.unwrap();

    trace::init_tracing(&state.config());

    tracing::info!(
        "Starting API server on http://{} with CORS allowed origins: {:?}",
        listener
            .local_addr()
            .map_err(|e| Error::Internal(e.to_string()))?,
        state.config().cors_allowed_origins()
    );

    axum::serve(listener, app_router.into_make_service())
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(())
}
