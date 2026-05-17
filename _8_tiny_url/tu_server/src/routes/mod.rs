use actix_web::web::ServiceConfig;

use crate::routes::{tinyurl::tinyurl_routers, users::users_routers};

pub mod tinyurl;
pub mod users;

pub fn view_routers(app: &mut ServiceConfig) {
    app.configure(users_routers).configure(tinyurl_routers);
}
