use actix_web::{HttpResponse, Responder, body::BoxBody, http::StatusCode};
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ExpenseTrackerErr {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Application Error")]
    AppError(AppError)
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        AppError { code, message: message.into() }
    }
}

impl Responder for AppError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // todo!()
        let body = serde_json::to_string(&ErrorResponse {
            message: self.message
        }).unwrap();

        HttpResponse::build(self.code)
            .content_type("application/json")
            .body(body)
    }
}