use actix_web::web::ServiceConfig;

use crate::routes::notify::notification_router;

pub mod notify;

pub fn view_routers(app: &mut ServiceConfig) {
    app.configure(notification_router);
}
