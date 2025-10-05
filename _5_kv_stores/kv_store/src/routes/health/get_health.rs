use std::sync::{atomic::AtomicUsize, Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::routes::health::Health;

pub async fn health_check(
    State(count): State<Arc<AtomicUsize>>
) -> impl IntoResponse {

    info!("->> GET   /health");
    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let visit_count = count.load(std::sync::atomic::Ordering::SeqCst);
    let health = Health {
        message: "Up and running..".to_string(),
        visit_count
    };
    
    (
        StatusCode::OK,
        Json(health)
    )
}