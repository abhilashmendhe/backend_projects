pub mod utils;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::utils::errors::BlogAppError;


pub async fn run() -> Result<(), BlogAppError> {

    let app = Router::new()
            .route("/", get("Hello world"));
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}