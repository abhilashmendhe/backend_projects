use actix_web::web::{self, ServiceConfig};

use crate::routes::notify::notify::notify_event;
pub mod notify;

pub fn notification_router(app: &mut ServiceConfig) {
    app.service(web::resource("/notify").route(web::post().to(notify_event)));
}
