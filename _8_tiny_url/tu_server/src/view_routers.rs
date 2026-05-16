use actix_web::web::{self, ServiceConfig};

pub fn views_factory(app: &mut ServiceConfig) {
    app.route(
        "/home",
        web::get().to(|| async { " Welcome to TinyUrl.com " }),
    );
}
