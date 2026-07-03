use actix_web::{App, HttpServer, web};

use crate::{
    utils::{app_state::AppState, errors::NotificationServerErr},
    view_router::views_factory,
};

pub mod make_connections;
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
            .configure(views_factory)
            .app_data(app_state.clone())
    })
    .workers(server_workers)
    .bind(format!("{}:{}", server_addr, port))?
    .run()
    .await?;

    Ok(())
}
