use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum BlogAppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Application Error")]
    AppError(AppError)
}

// ------------------------ App Error --------------------------

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_message: String
}

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (   self.code, 
            Json(ErrorResponse{ error_message: self.message.clone()})
        ).into_response()
    }
}