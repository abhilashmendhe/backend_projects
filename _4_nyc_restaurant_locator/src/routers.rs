use axum::{http::{self, Method, StatusCode}, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use crate::{routes::fetch_all_restaurants::fetch_all_restaurants, utils::app_state::AppState};

pub async fn create_router(app_state: AppState) -> Router {

    // cors
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    Router::new()

        .route("/v1/restaurants", get(fetch_all_restaurants))
        .layer(cors)
        .route("/", get(
            (StatusCode::OK, 
                "Welcome to my NYC restaurant finder web app service!")
        ))
        .with_state(app_state)
}