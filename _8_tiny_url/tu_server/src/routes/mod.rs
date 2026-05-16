use actix_web::web::ServiceConfig;

use crate::routes::users::users_routers;

pub mod users;

pub fn view_routers(app: &mut ServiceConfig) {
    app.configure(users_routers);
}
