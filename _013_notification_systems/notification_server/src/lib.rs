use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

use crate::{
    utils::{app_state::AppState, errors::NotificationServerErr},
    view_router::views_factory,
};

pub mod make_connections;
pub mod models;
pub mod routes;
pub mod utils;
pub mod view_router;

pub async fn run(
    server_addr: &str,
    port: u16,
    server_workers: usize,
    app_state: web::Data<AppState>,
) -> Result<(), NotificationServerErr> {
    HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(not_found))
            .configure(views_factory)
            .app_data(app_state.clone())
    })
    .workers(server_workers)
    .bind(format!("{}:{}", server_addr, port))?
    .run()
    .await?;

    Ok(())
}

async fn not_found(req: HttpRequest) -> impl Responder {
    let path = req.path();
    let method = req.method();
    tracing::error!("-->\t {method} {path} NOT FOUND");
    #[derive(Serialize)]
    struct ApiError {
        status: u16,
        error: String,
        message: String,
    }

    HttpResponse::NotFound().json(ApiError {
        status: 404,
        error: "Not Found".to_string(),
        message: "Route does not exist".to_string(),
    })
}
