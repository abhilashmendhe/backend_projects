use axum::{routing::get, Router};

use crate::{routes::{health::get_health::health_check, timekv::{get_value::get_value, put_value::put_value}}, utils::app_state::AppState};

pub fn routers(app_state: AppState) -> Router {

    Router::new()
    
        .route("/", get(get_value)
                                        .put(put_value))

        .route("/health", get(health_check))
        .with_state(app_state)
}
