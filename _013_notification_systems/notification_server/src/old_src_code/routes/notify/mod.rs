use actix_web::web::{self, ServiceConfig};

use crate::routes::notify::notify::notify_event;

pub mod notification_retry;
pub mod notify;
pub mod push_to_queue;

pub fn notification_router(app: &mut ServiceConfig) {
    app.service(web::resource("/notify").route(web::post().to(notify_event)));
}
