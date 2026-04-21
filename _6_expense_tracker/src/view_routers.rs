use std::sync::atomic::Ordering;

use actix_web::{HttpResponse, web::{self, ServiceConfig, scope}};
use serde::Serialize;

use crate::{routes::users_view_factory, utils::{app_state::AppState, errors::ExpenseTrackerErr}};


pub fn views_factory(app: &mut ServiceConfig) {
     app.service(
        scope("/v1")
            .route("/health-check", web::get().to(health_check))
            .configure(users_view_factory)
    );
}

#[derive(Debug, Serialize)]
struct HealthCheckResponse {
    visit_count: usize,
    message: String
}

async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse, ExpenseTrackerErr> {
    tracing::info!("GET /v1/health-check health check hit");
    let value = app_state.visit_count.fetch_add(1, Ordering::Relaxed) + 1;
    // let value = app_state.visit_count.load(std::sync::atomic::Ordering::Relaxed); // only fetches the value
    let hc_resp = HealthCheckResponse {
        visit_count: value, 
        message: format!("Welcome. System up running for {:?}",app_state.service_up.elapsed())
    };
    // Ok(HttpResponse::Ok().body(format!("Works! Visit count: {}\n",value)))
    Ok(HttpResponse::Ok().json(hc_resp))
}