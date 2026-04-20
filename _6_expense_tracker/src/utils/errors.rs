use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ExpenseTrackerErr {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("VarError: {0}")]
    VarError(#[from] std::env::VarError),

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

impl ResponseError for ExpenseTrackerErr {
    fn status_code(&self) -> StatusCode {
        match self {
            ExpenseTrackerErr::Io(_) =>  StatusCode::INTERNAL_SERVER_ERROR,
            ExpenseTrackerErr::VarError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ExpenseTrackerErr::AppError(app_error) => app_error.code
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = match self {
            ExpenseTrackerErr::Io(err) => err.to_string(),
            ExpenseTrackerErr::VarError(err) => err.to_string(),
            ExpenseTrackerErr::AppError(app_err) => app_err.message.clone(),
        };

        HttpResponse::build(self.status_code()).json(ErrorResponse { message })
    }
}

// impl Responder for AppError {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
//         // todo!()
//         let body = serde_json::to_string(&ErrorResponse {
//             message: self.message
//         }).unwrap();

//         HttpResponse::build(self.code)
//             .content_type("application/json")
//             .body(body)
//     }
// }

