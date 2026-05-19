use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig, scope},
};

use crate::{
    middleware::require_auth::require_auth,
    routes::users::{
        create_user::create_user, delete_user::delete_user, get_user::get_user, login::login,
        logout::logout,
    },
};

pub mod create_user;
pub mod delete_user;
pub mod get_user;
pub mod login;
pub mod logout;

pub fn users_routers(app: &mut ServiceConfig) {
    app.service(
        scope("/user")
            .route("", web::post().to(create_user))
            .route("/login", web::post().to(login))
            .service(
                web::resource("/logout")
                    .wrap(from_fn(require_auth))
                    .route(web::post().to(logout)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(from_fn(require_auth))
                    .route(web::get().to(get_user))
                    .route(web::delete().to(delete_user)),
            ),
    );
}
