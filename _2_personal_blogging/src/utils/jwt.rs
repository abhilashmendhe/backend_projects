use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::errors::AppError;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String
}

pub async fn create_token(
    secret: &str, 
    username: String
) -> Result<String, AppError> {

    let now = chrono::Utc::now();
    let expires_at = chrono::Duration::hours(1);
    let exp = (now + expires_at).timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key)
        .map_err(|err| {
            eprintln!("Error creating JWT token: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "There was an error, please try again later."
            )
        })
}

pub async fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {

    decode::<Claims>(
        token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(jsonwebtoken::Algorithm::HS256)
    ).map_err(|err| {
        eprintln!("Error validating token: {:?}", err);
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken 
            | jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                AppError::new(
                    StatusCode::UNAUTHORIZED, 
                    "Not authenticated."
                )
            }
            _=> {
                eprintln!("Error validating token: {:?}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        }
    })
    .map(|_|true)
}