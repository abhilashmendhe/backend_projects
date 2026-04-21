use actix_web::{App, HttpServer, web::Data};
use tracing_actix_web::TracingLogger;

use crate::{utils::{app_state::AppState, errors::ExpenseTrackerErr}, view_routers::views_factory};

pub mod utils;
pub mod view_routers;
pub mod routes;
pub mod models;
pub mod middleware; 

pub async fn run(app_state: Data<AppState>) -> Result<(), ExpenseTrackerErr> {

    HttpServer::new(move ||{
        App::new()
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            .configure(views_factory)
    })
    .workers(2)
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}