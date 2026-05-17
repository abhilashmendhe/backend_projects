use axum::{
    Router,
    routing::{get, post},
};

use crate::{routes::{insert_key::insert_key, query_key::query_key, save::save}, utils::app_state::AppState};

pub fn create_routers(app_state: AppState) -> Router {
    Router::new()
        .route("/bf/v1", post(insert_key))
        .route("/bf/v1", get(query_key))
        .route("/bf/v1/save", post(save))
        .with_state(app_state)
}
