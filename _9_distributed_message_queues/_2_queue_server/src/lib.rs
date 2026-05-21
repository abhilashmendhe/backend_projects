use actix_web::{App, HttpServer, web};
use chrono::Utc;

use crate::{
    models::payload_req::PayloadReq,
    services::start_process_workers::start_process_workers,
    utils::{app_state::AppState, errors::QueueServerErr},
    view_routers::views_factory,
};

pub mod models;
pub mod routes;
pub mod services;
pub mod utils;
pub mod view_routers;

pub async fn run(
    addr: &str,
    port: u16,
    num_acx_servers: usize,
    num_process_workers: usize,
    rx: tokio::sync::mpsc::Receiver<PayloadReq>,
    app_state: web::Data<AppState>,
) -> Result<(), QueueServerErr> {
    // 1. Start recv from rx
    start_process_workers(num_process_workers, rx).await;

    // 2. Run http server
    HttpServer::new(move || {
        App::new()
            .configure(views_factory)
            .app_data(app_state.clone())
    })
    .workers(num_acx_servers)
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await?;

    Ok(())
}
