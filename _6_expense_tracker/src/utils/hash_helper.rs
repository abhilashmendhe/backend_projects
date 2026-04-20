use actix_web::http::StatusCode;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

use crate::utils::errors::{AppError, ExpenseTrackerErr};

pub fn hash_password(password: &str) -> Result<String, ExpenseTrackerErr> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err|{
            tracing::error!("Failed to hash the password: {:?}",err);
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error."))
        }).
        map(|f| f.to_string())
}

pub fn verify_hash_password(password: &str, hashed_password: &str) -> Result<(), ExpenseTrackerErr> {

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|err|{
            tracing::error!("Failed to parse the hashed password: {:?}",err);
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error."))
        }).
        map(|f| f)?;
    argon2.verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err|{
            tracing::error!("Failed to verify the hashed password: {:?}",err);
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::BAD_REQUEST, "Password verification failed!"))
        })
}