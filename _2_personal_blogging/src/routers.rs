use std::sync::{atomic::AtomicU32, Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};

use crate::utils::app_state::AppState;

pub fn create_router(app_state: AppState) -> Router {

    Router::new()
        .route("/", get("Hello world"))
        .route("/health", get(health_check))
        .with_state(app_state)
}

pub async fn health_check(
    State(count): State<Arc<AtomicU32>>
) -> impl IntoResponse {
    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let value = count.load(std::sync::atomic::Ordering::SeqCst);
    (StatusCode::OK, format!("Works! Visit count: {}\n", value))
}