use std::sync::atomic::Ordering;

use actix_web::{HttpResponse, web::{self, ServiceConfig, scope}};

use crate::utils::app_state::AppState;


pub fn views_factory(app: &mut ServiceConfig) {
    app.service(
scope("/v1")
            .route("/health-check", web::get().to(health_check))
    );
}

async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let value = app_state.visit_count.fetch_add(1, Ordering::Relaxed) + 1;
    // let value = app_state.visit_count.load(std::sync::atomic::Ordering::Relaxed); // only fetches the value
    HttpResponse::Ok().body(format!("Works! Visit count: {}\n",value))
}