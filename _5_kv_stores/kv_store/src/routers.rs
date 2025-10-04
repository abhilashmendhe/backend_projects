use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use tokio::sync::Mutex;

use crate::utils::app_state::AppState;

pub fn routers(app_state: AppState) -> Router {

    Router::new()
        .route("/health", get(health_check))
        .with_state(app_state)
}

async fn health_check(
    State(count): State<Arc<Mutex<u64>>>
) -> impl IntoResponse {

    let value = {
        let mut count_lock_gaurd = count.lock().await;
        *count_lock_gaurd += 1;
        *count_lock_gaurd
    };
    (StatusCode::OK, format!("Healty... Visit count: {}\n", value)).into_response()
}