use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

use crate::{
    utils::{app_state::AppState, errors::IngestionServiceErr},
    view_router::views_factory,
};

pub async fn start_server(
    server_workers: usize,
    server_addr: &str,
    port: u16,
    app_state: web::Data<AppState>,
) -> Result<(), IngestionServiceErr> {
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .default_service(web::route().to(not_found))
            .configure(views_factory)
    })
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
