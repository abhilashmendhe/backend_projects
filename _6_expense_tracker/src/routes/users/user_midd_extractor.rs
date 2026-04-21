use std::future::{Ready, ready};

use actix_web::{Error, FromRequest, HttpMessage, error::ErrorUnauthorized};

use crate::models::users_model::UserModel;

impl FromRequest for UserModel {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        
        let binding = req.extensions();
        let user = binding.get::<UserModel>();
        match user {
            Some(u) => ready(Ok(u.clone())),
            None => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}