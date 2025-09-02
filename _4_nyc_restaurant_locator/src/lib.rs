use tokio::net::TcpListener;
use tracing::info;

use crate::{routers::create_router, utils::{app_state::AppState, errors::WebError}};

pub mod utils;
pub mod routers;
pub mod routes;
pub mod models;

pub async fn run(
    address: String,
    app_state: AppState
) -> Result<(), WebError> {

    let listener = TcpListener::bind(&address).await?;
    info!("🚀 Server listening on http://{}", address);

    let app = create_router(app_state).await;

    axum::serve(listener, app)
        .await?;

    Ok(())
}