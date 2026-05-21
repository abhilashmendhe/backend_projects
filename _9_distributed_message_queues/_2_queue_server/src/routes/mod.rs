use actix_web::web::{ServiceConfig, post};

use crate::routes::process_payload_request::process_payload_request;

pub mod process_payload_request;

pub fn request_handler(app: &mut ServiceConfig) {
    app.route("", post().to(process_payload_request));
}
