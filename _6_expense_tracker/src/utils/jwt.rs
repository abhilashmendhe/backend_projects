use actix_web::http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::utils::errors::{AppError, ExpenseTrackerErr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize, 
    username: String
}

pub async fn create_token(
    secret: &str, 
    username: String
) -> Result<String, ExpenseTrackerErr> {

    let now = chrono::Utc::now();
    let expires_at = chrono::Duration::hours(1);
    let exp = (now + expires_at).timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    
    encode(&token_header, &claims, &key)
        .map_err(|err| {
            tracing::error!("Error creating JWT token: {:?}",err);
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later"))
        })
}

pub async fn validate_token(
    secret: &str,
    token: &str
) -> Result<bool, ExpenseTrackerErr> {
    decode::<Claims>(
        token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
            &Validation::new(jsonwebtoken::Algorithm::HS256)
    ).map_err(|err| {
        tracing::error!("Error validating JWT token: {:?}",err);
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken | jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                ExpenseTrackerErr::AppError(AppError::new(StatusCode::UNAUTHORIZED, "Not Authenticated."))
            },
            _ => {
                ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server failed to validate token."))
            }
        }
    })
    .map(|_| true)
}