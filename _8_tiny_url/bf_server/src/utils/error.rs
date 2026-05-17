use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BloomFilterErr {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("BF error: {0}")]
    BloomFilterErr(#[from] bloomfilter::errors::BFError),

    #[error("BF Custom Error")]
    BFCustomErr(AppError),
}

// ---- App Err ----

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error_message: self.message,
            }),
        )
            .into_response()
    }
}

impl IntoResponse for BloomFilterErr {
    fn into_response(self) -> axum::response::Response {
        match self {
            BloomFilterErr::BFCustomErr(app_err) => app_err.into_response(),

            other => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled error: {}", other),
            )
                .into_response(),
        }
    }
}
