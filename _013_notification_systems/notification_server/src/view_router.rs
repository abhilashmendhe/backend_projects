use actix_web::{
    HttpResponse, Responder,
    web::{self, ServiceConfig},
};
use serde::{Deserialize, Serialize};

use crate::utils::app_state::AppState;

pub fn views_factory(app: &mut ServiceConfig) {
    app.route("/about", web::get().to(about));
}

pub async fn about(app_data: web::Data<AppState>) -> impl Responder {
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
