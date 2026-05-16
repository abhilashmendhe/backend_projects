use actix_web::http::StatusCode;
use argon2::{
    PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::utils::errors::{AppError, TinyUrlError};

pub fn hash_password(password: &str) -> Result<String, TinyUrlError> {
    let argon2 = argon2::Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let result = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_err| {
            tracing::error!("Failed to hash password: {:?}", _err);
            return TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error.",
            ));
        })?;
    Ok(result.to_string())
}

pub fn verify_hash_password(password: &str, hashed_password: &str) -> Result<(), TinyUrlError> {
    let argon2 = argon2::Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|err| {
            tracing::error!("Failed to parse the hashed password: {:?}", err);
            return TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ));
        })
        .map(|f| f)?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| {
            tracing::error!("Failed to verify the hashed password: {:?}", err);
            return TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ));
        })?;
    Ok(())
}
