use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    // iat: usize
}

pub fn create_token(secret: &str) -> Result<String, AppError> {

    // add at least an hour for this timestamp 
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&token_header, &claims, &key)
        .map_err(|e|{
            eprintln!("Error creating JWT token: {:?}", e);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "There was an error, please try again later."
            )
        })
}