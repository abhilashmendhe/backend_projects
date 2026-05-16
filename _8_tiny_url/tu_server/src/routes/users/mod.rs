use actix_web::web::{self, ServiceConfig, scope};

use crate::routes::users::{create_user::create_user, login::login};

pub mod create_user;
pub mod login;

pub fn users_routers(app: &mut ServiceConfig) {
    app.service(
        scope("/user")
            .route("", web::post().to(create_user))
            .route("/login", web::post().to(login)),
    );
}
