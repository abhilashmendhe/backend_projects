use std::{env::VarError, num::ParseIntError};

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TinyUrlError {
    #[error("{}", .0)]
    EnvVarErr(#[from] VarError),

    #[error("{}", .0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", .0)]
    IOErr(#[from] std::io::Error),

    #[error("{}", .0)]
    RedisErr(#[from] redis::RedisError),

    #[error("Application Error")]
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

impl ResponseError for TinyUrlError {
    fn status_code(&self) -> StatusCode {
        // StatusCode::INTERNAL_SERVER_ERROR
        match self {
            TinyUrlError::EnvVarErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TinyUrlError::ParseIntError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TinyUrlError::IOErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TinyUrlError::AppError(app_error) => app_error.code,
            TinyUrlError::RedisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            TinyUrlError::EnvVarErr(var_error) => var_error.to_string(),
            TinyUrlError::ParseIntError(parse_int_error) => parse_int_error.to_string(),
            TinyUrlError::IOErr(error) => error.to_string(),
            TinyUrlError::AppError(app_error) => app_error.message.to_string(),
            TinyUrlError::RedisErr(redis_error) => redis_error.to_string(),
        };
        HttpResponse::build(self.status_code()).json(ErrorResponse { message })
    }
}
