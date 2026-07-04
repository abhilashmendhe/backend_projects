use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self, ServiceConfig},
};
use serde::{Deserialize, Serialize};

use crate::{routes::view_routers, utils::app_state::AppState};

pub fn views_factory(app: &mut ServiceConfig) {
    app.route("/about", web::get().to(about))
        .configure(view_routers);
}

pub async fn about(req: HttpRequest, app_data: web::Data<AppState>) -> impl Responder {
    let path = req.path();
    let method = req.method();
    tracing::info!("-->\t {method} {path}");
    #[derive(Deserialize, Serialize)]
    struct AboutResponse {
        message: String,
    }
    HttpResponse::Ok().json(AboutResponse {
        message: format!(
            "I am notification server running on ::{}",
            &app_data.config().port()
        ),
    })
}
