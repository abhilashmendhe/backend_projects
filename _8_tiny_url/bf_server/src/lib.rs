use tokio::signal;

use crate::{
    router::create_routers,
    utils::{app_state::AppState, error::BloomFilterErr},
};

pub mod router;
pub mod routes;
pub mod utils;

pub async fn run(addr: &str, app_state: AppState) -> Result<(), BloomFilterErr> {
    // 1. Bind to address
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🚀 Server listening on http://{}", addr);

    // 2. create app
    let app = create_routers(app_state);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Shutdown signal func to stop the server gracefully
async fn shutdown_signal() {
    // Handles ctrl+c in local dev
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal;

        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🛑 Shutdown signal received, stopping server...");
}
