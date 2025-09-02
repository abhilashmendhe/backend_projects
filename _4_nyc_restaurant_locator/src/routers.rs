use axum::{http::StatusCode, routing::get, Router};

use crate::utils::app_state::AppState;

pub async fn create_router(app_state: AppState) -> Router {

    Router::new()
        .route("/", get(
            (StatusCode::OK, "Welcome to my NYC restaurant finder web app service!")
        ))
        .with_state(app_state)
}