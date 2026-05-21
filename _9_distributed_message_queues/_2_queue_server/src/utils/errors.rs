use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueServerErr {
    #[error("{}", .0)]
    IoErr(#[from] std::io::Error),

    #[error("{}", .0)]
    RedisErr(#[from] redis::RedisError),

    #[error("Queue server error")]
    AppError(AppError),
}

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
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

impl ResponseError for QueueServerErr {
    fn status_code(&self) -> StatusCode {
        match self {
            QueueServerErr::IoErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QueueServerErr::AppError(app_error) => app_error.code,
            QueueServerErr::RedisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            QueueServerErr::IoErr(error) => error.to_string(),
            QueueServerErr::AppError(app_error) => app_error.message.to_string(),
            QueueServerErr::RedisErr(redis_error) => redis_error.to_string(),
        };
        HttpResponse::build(self.status_code()).json(ErrorResponse { message })
    }
}
