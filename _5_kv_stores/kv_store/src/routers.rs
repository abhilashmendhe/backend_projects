use axum::{routing::get, Router};

use crate::{routes::health::get_health::health_check, utils::app_state::AppState};

pub fn routers(app_state: AppState) -> Router {

    Router::new()
        .route("/health", get(health_check))
        .with_state(app_state)
}
