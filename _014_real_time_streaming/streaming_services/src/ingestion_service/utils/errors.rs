use std::net::AddrParseError;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use prost::EncodeError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestionServiceErr {
    #[error("{}", 0)]
    IoErr(#[from] std::io::Error),

    #[error("{}", 0)]
    AddrParseErr(#[from] AddrParseError),

    #[error("{}", 0)]
    TonicTransportErr(#[from] tonic::transport::Error),

    #[error("{}", 0)]
    GrpcEncode(#[from] EncodeError),

    #[error("Ingestion server error.")]
    AppError(AppError),
}

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code, message }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for IngestionServiceErr {
    fn status_code(&self) -> StatusCode {
        match self {
            IngestionServiceErr::IoErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IngestionServiceErr::AddrParseErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IngestionServiceErr::TonicTransportErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IngestionServiceErr::GrpcEncode(_) => StatusCode::INTERNAL_SERVER_ERROR,
            IngestionServiceErr::AppError(app_err) => app_err.code,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let message = match self {
            IngestionServiceErr::IoErr(error) => error.to_string(),
            IngestionServiceErr::AddrParseErr(addr_parse_error) => addr_parse_error.to_string(),
            IngestionServiceErr::TonicTransportErr(error) => error.to_string(),
            IngestionServiceErr::GrpcEncode(encode_error) => encode_error.to_string(),
            IngestionServiceErr::AppError(app_error) => app_error.message.clone(),
        };
        HttpResponse::build(self.status_code()).json(ErrorResponse { message })
    }
}
