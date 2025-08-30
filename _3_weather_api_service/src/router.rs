use std::{sync::{atomic::AtomicU32, Arc}};

use axum::{extract::State, http::{self, StatusCode}, middleware, response::IntoResponse, routing::get, Router};
use tokio::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

use crate::{middleware::rate_limit::rate_limit, routes::get_weather_by_location::get_weather_by_location, utils::app_state::AppState};

pub fn create_routers(app_state: AppState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);
    
    Router::new()
        .route("/v1/weather", get(get_weather_by_location))
        .layer(cors)
        .route_layer(middleware::from_fn_with_state(app_state.clone(), rate_limit))
        .route("/health_check", get(health_check))
        .with_state(app_state)
}

pub async fn health_check(
    State(count): State<Arc<AtomicU32>>,
    State(running): State<Instant>,
) -> impl IntoResponse {
    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let value = count.load(std::sync::atomic::Ordering::SeqCst);
    let output = format!("Welcome to our weather service app. Visit count:{}\nUp running: {} secs.\n",value,running.elapsed().div_f64(1000.0).as_millis());
    (
        StatusCode::OK,
        output
    ).into_response()
}