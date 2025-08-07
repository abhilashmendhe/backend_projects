pub mod router;
pub mod routes;
pub mod database;
pub mod utils;
pub mod middleware;

use tokio::net::TcpListener;

use crate::{router::create_router, utils::app_state::AppState};

pub async fn run(app_state: AppState) {
    

    let app = create_router(app_state);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}