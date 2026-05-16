use actix_web::web::{self, ServiceConfig, scope};

use crate::routes::view_routers;

pub fn views_factory(app: &mut ServiceConfig) {
    app.route(
        "/home",
        web::get().to(|| async { " Welcome to TinyUrl.com " }),
    )
    .service(scope("/v1").configure(view_routers));
}
