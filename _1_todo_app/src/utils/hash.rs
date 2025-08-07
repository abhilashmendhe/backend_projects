use axum::http::StatusCode;
use bcrypt::hash;

use crate::utils::app_error::AppError;

pub fn hash_password(
    password: &str
) -> Result<String, AppError> {
    hash(password, 4)
        .map_err(|err|{
            eprintln!("Error hashing password: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Error securing password")
        })
}