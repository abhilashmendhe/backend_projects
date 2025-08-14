pub mod utils;
pub mod routers;
pub mod routes;
pub mod models;

use tokio::net::TcpListener;
use crate::{routers::create_router, utils::{app_state::AppState, errors::BlogAppError}};

pub async fn run(app_state: AppState) -> Result<(), BlogAppError> {

    let app = create_router(app_state);
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}