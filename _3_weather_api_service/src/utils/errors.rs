use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeatherServiceErr {

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("VarError: {0}")]
    VarErr(#[from] std::env::VarError),

    #[error("Reqwest Err: {0}")]
    ReqwestErr(#[from] reqwest::Error),
    
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("serde_json error: {0}")]
    SerdeJSONErr(#[from] serde_json::Error),
    
    #[error("Web Server Error")]
    WebServerErr(WebServerErr)
}   


#[derive(Debug)]
pub struct WebServerErr {
    code: StatusCode,
    message: String
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_message: String
}


impl WebServerErr {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code, 
            message: message.into()
        }
    }
}

impl IntoResponse for WebServerErr {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse{error_message: self.message.clone()})
        ).into_response()
    }
}

impl IntoResponse for WeatherServiceErr {
    fn into_response(self) -> axum::response::Response {
        match self {
            WeatherServiceErr::Io(error) => (error.to_string()).into_response(),
            WeatherServiceErr::VarErr(error) => (error.to_string()).into_response(),
            WeatherServiceErr::ReqwestErr(error) => (error.to_string()).into_response(),
            WeatherServiceErr::RedisError(error) => (error.to_string()).into_response(),
            WeatherServiceErr::SerdeJSONErr(error) => (error.to_string()).into_response(),
            WeatherServiceErr::WebServerErr(web_server_err) => (
                    web_server_err.code,
                    Json(ErrorResponse{error_message: web_server_err.message.clone()})
                ).into_response(),
        }
    }
}