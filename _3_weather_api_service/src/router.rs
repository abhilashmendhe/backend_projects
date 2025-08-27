use std::sync::{atomic::AtomicU32, Arc};

use axum::{extract::State, response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;

use crate::utils::app_state::AppState;

pub fn create_routers(app_state: AppState) -> Router {

    Router::new()
        .route("/health_check", get(health_check))
        .with_state(app_state)
}

pub async fn health_check(
    State(count): State<Arc<AtomicU32>>
) -> impl IntoResponse {
    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let value = count.load(std::sync::atomic::Ordering::SeqCst);
    (
        StatusCode::OK,
        format!("Welcome to our weather service app. Visit count: {}\n", value)
    )
}