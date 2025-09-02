use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum WebError {

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Application Error")]
    AppError(AppError)

}

// AppError
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

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        match self {
            WebError::IOError(error) => {
                error!("IO error: {:?}",error);
                (error.to_string()).into_response()
            },
            WebError::AppError(app_error) => (   
                app_error.code, 
                Json(ErrorResponse{ error_message: app_error.message.clone() }
            )
        ).into_response(),
        }
    }
}