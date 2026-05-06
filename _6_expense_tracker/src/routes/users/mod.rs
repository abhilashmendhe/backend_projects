use crate::{
    middleware::require_auth::require_auth,
    routes::users::{
        activate_acc::account_activate, create_user::create_user, delete_user::delete_user,
        get_user::get_user, login::login, logout::logout, update_password::update_password,
    },
};
use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig, scope},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod activate_acc;
pub mod create_user;
pub mod delete_user;
pub mod get_user;
pub mod login;
pub mod logout;
pub mod update_password;
pub mod user_midd_extractor;

#[derive(Debug, Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseUser {
    pub id: i32,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
    token: Option<String>,
}

pub fn users_view_factory(app: &mut ServiceConfig) {
    // let protected = web::scope("")
    //         .wrap(from_fn(require_auth))
    //         .route("/{id}", web::delete().to(delete_user))
    //         .route("/logout", web::post().to(logout))
    //         .route("/update-password", web::put().to(update_password));

    // app.service(
    //     scope("/user")
    //         .service(protected)
    //         .route("/login", web::post().to(login))
    //         .route("/{id}", web::get().to(get_user))
    //         .route("", web::post().to(create_user))
    //         .route("/activate", web::put().to(account_activate))
    // );

    app.service(
        scope("/user")
            .route("/login", web::post().to(login))
            .route("", web::post().to(create_user))
            .route("/activate", web::put().to(account_activate))
            // 🔐 Protected routes
            .service(
                // web::resource("/{id}")
                web::resource("")
                    .wrap(from_fn(require_auth))
                    .route(web::delete().to(delete_user))
                    .route(web::get().to(get_user)),
            )
            .service(
                web::resource("/logout")
                    .wrap(from_fn(require_auth))
                    .route(web::post().to(logout)),
            )
            .service(
                web::resource("/update-password")
                    .wrap(from_fn(require_auth))
                    .route(web::put().to(update_password)),
            ),
    );
}
