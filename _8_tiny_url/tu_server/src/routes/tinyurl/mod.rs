use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig},
};

use crate::{
    middleware::require_auth::require_auth,
    routes::tinyurl::{create_short_url::create_short_url, get_short_url::get_redirect_short_url},
};

const TURL: &'static str = "http://mybitiny.com";

pub mod bloom_filter_insert;
pub mod bloom_filter_query;
pub mod create_short_url;
pub mod get_short_url;
pub mod helpers;

pub fn tinyurl_routers(app: &mut ServiceConfig) {
    app.service(
        web::resource("/tinyurl")
            .wrap(from_fn(require_auth))
            .route(web::post().to(create_short_url)),
    )
    .service(
        web::resource("/tinyurl/{key}")
            .wrap(from_fn(require_auth))
            .route(web::get().to(get_redirect_short_url)),
    );
}
