use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;

use crate::{models::events_model::Event, utils::app_state::AppState};

pub async fn req_post_events(
    req: HttpRequest,
    req_event: web::Json<Event>,
    _app_data: web::Data<AppState>,
) -> impl Responder {
    tracing::info!(
        method = %req.method(),
        path = %req.path(),
        device_id = %req_event.device_id,
        room_id = %req_event.room_id,
        seq = req_event.seq,
        "received event"
    );
    #[derive(Debug, Serialize)]
    struct EventResponse {
        ok: bool,
    }
    HttpResponse::Accepted().json(EventResponse { ok: true })
}
