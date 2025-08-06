use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::routes::hello_world::hello_world;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
}

pub async fn run() {
    let app = create_router();
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}