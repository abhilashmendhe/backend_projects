use std::{env::VarError, num::ParseIntError};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use redis::RedisError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationServerErr {
    #[error("{}", 0)]
    IOErr(#[from] std::io::Error),

    #[error("{}", 0)]
    EnvVarErr(#[from] VarError),

    #[error("{}", 0)]
    RedisErr(#[from] RedisError),

    #[error("{}", 0)]
    ParseIntErr(#[from] ParseIntError),

    #[error("Notification server error")]
    AppError(AppError),
}

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            code,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for NotificationServerErr {
    fn status_code(&self) -> StatusCode {
        match self {
            NotificationServerErr::IOErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotificationServerErr::EnvVarErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotificationServerErr::RedisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotificationServerErr::ParseIntErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotificationServerErr::AppError(app_error) => app_error.code,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            NotificationServerErr::IOErr(error) => error.to_string(),
            NotificationServerErr::EnvVarErr(var_error) => var_error.to_string(),
            NotificationServerErr::RedisErr(redis_error) => redis_error.to_string(),
            NotificationServerErr::ParseIntErr(parse_int_error) => parse_int_error.to_string(),
            NotificationServerErr::AppError(app_error) => app_error.message.to_string(),
        };
        HttpResponse::build(self.status_code()).json(ErrorResponse { message })
    }
}
