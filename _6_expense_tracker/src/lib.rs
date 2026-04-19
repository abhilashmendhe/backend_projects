use actix_web::{App, HttpServer, web::Data};

use crate::{routes::views_factory, utils::{app_state::AppState, errors::ExpenseTrackerErr}};

pub mod utils;
pub mod routes;

pub async fn run(app_state: Data<AppState>) -> Result<(), ExpenseTrackerErr> {

    HttpServer::new(move ||{
        App::new()
            .app_data(app_state.clone())
            .configure(views_factory)
    }).bind("0.0.0.0:8080").unwrap().run().await?;

    Ok(())
}