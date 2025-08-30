use std::sync::{atomic::AtomicU32, Arc};

use axum::{http::StatusCode, extract::State, http, response::IntoResponse, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use http::Method;

use crate::{routes::get_weather_by_location::get_weather_by_location, utils::app_state::AppState};

pub fn create_routers(app_state: AppState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()
        .route("/v1/weather", get(get_weather_by_location))
        .route("/health_check", get(health_check))
        .layer(cors)
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