use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self, ServiceConfig},
};
use serde::Serialize;

use crate::{routes::view_routers, utils::app_state::AppState};

pub fn views_factory(app: &mut ServiceConfig) {
    app.route("/about", web::get().to(about))
        .configure(view_routers);
}

pub async fn about(req: HttpRequest, app_data: web::Data<AppState>) -> impl Responder {
    tracing::info!(
        method = %req.method(),
        path = %req.path(),
        "requested about"
    );

    #[derive(Debug, Serialize)]
    struct AboutResponse {
        message: String,
        visit_count: u64,
        alive_time_secs: f64,
    }

    HttpResponse::Ok().json(AboutResponse {
        message: "I am ingestion service".to_string(),
        visit_count: app_data.visit_count(),
        alive_time_secs: app_data.alive_time().elapsed().as_secs_f64(),
    })
}
