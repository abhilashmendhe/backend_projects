use actix_web::{App, HttpServer, web};

use crate::{
    utils::{app_state::AppState, errors::TinyUrlError},
    view_routers::views_factory,
};

pub mod utils;
pub mod view_routers;

pub async fn run(
    localhost: String,
    port: u16,
    server_workers: usize,
    app_state: web::Data<AppState>,
) -> Result<(), TinyUrlError> {
    HttpServer::new(move || {
        App::new()
            .configure(views_factory)
            .app_data(app_state.clone())
    })
    .workers(server_workers)
    .bind(format!("{}:{}", localhost, port))?
    .run()
    .await?;

    Ok(())
}
