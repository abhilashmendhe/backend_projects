use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

use crate::utils::errors::IngestionServiceErr;

pub async fn start_server(
    server_workers: usize,
    server_addr: &str,
    port: u16,
) -> Result<(), IngestionServiceErr> {
    HttpServer::new(move || App::new().default_service(web::route().to(not_found)))
        .workers(server_workers)
        .bind(format!("{}:{}", server_addr, port))?
        .run()
        .await?;

    Ok(())
}

pub async fn not_found(req: HttpRequest) -> impl Responder {
    let path = req.path();
    let method = req.method();
    tracing::error!("-->\t {method} {path} NOT FOUND");

    #[derive(Serialize)]
    struct ApiErr {
        status: u16,
        error: String,
        message: String,
    }

    HttpResponse::NotFound().json(ApiErr {
        status: 404,
        error: "Not Found".to_string(),
        message: "Route doesn't exists".to_string(),
    })
}
