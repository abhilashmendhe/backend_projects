use actix_web::web::{self, ServiceConfig};

use crate::routes::ingestion::{
    get_alarms::get_alarms, get_device::get_device_health, get_room::get_room_occupancy,
    post_events::req_post_events,
};

pub mod get_alarms;
pub mod get_device;
pub mod get_room;
pub mod post_events;

pub fn ingest_route(app: &mut ServiceConfig) {
    app.route("/events", web::post().to(req_post_events))
        .route(
            "/devices/{device_id}/health",
            web::get().to(get_device_health),
        )
        .route("/alarms", web::get().to(get_alarms))
        .route(
            "/rooms/{room_id}/occupancy",
            web::get().to(get_room_occupancy),
        );
}
