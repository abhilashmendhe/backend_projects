use actix_web::web::{ServiceConfig, scope};

use crate::routes::request_handler;

pub fn views_factory(app: &mut ServiceConfig) {
    app.service(scope("/v1/queue-server").configure(request_handler));
}
