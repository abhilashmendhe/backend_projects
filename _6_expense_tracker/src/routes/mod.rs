use actix_web::{middleware::from_fn, web::{self, ServiceConfig, scope}};

use crate::{middleware::require_auth::require_auth, routes::users::{activate_acc::account_activate, create_user::create_user, delete_user::delete_user, get_user::get_user, login::login, logout::logout, update_password::update_password}};

pub mod users;

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
                    .route(web::get().to(get_user))
            )
            .service(
                web::resource("/logout")
                    .wrap(from_fn(require_auth))
                    .route(web::post().to(logout))
            )
            .service(
                web::resource("/update-password")
                    .wrap(from_fn(require_auth))
                    .route(web::put().to(update_password))
            )
            
    );
}