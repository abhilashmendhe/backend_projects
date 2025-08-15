use std::sync::{atomic::AtomicU32, Arc};

use axum::{extract::State, http::StatusCode, middleware, response::IntoResponse, routing::{get, post}, Router};

use crate::{middleware::require_auth::require_authentication, routes::users::{create_user::create_user, login::login, logout::logout}, utils::app_state::AppState};

pub fn create_router(app_state: AppState) -> Router {

    Router::new()
        .route("/v1/users/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication))
        .route("/v1/users/login", post(login))
        .route("/v1/users", post(create_user))
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