use axum::{http::StatusCode, routing::get, Router};

use crate::{routes::fetch_all_restaurants::fetch_all_restaurants, utils::app_state::AppState};

pub async fn create_router(app_state: AppState) -> Router {

    Router::new()

        .route("/v1/restaurants", get(fetch_all_restaurants))

        .route("/", get(
            (StatusCode::OK, 
                "Welcome to my NYC restaurant finder web app service!")
        ))
        .with_state(app_state)
}