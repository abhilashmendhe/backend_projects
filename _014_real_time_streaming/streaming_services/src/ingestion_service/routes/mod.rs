use actix_web::web::ServiceConfig;

use crate::routes::ingestion::ingest_route;

pub mod ingestion;

pub fn view_routers(app: &mut ServiceConfig) {
    app.configure(ingest_route);
}
