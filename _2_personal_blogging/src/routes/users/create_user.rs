use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify};
use sqlx::PgPool;
use sqlx::Row;

use crate::{routes::users::{RequestUser, ResponseUser}, utils::errors::AppError};

pub async fn create_user(
    State(db): State<PgPool>,
    Json(req_user): Json<RequestUser>
) -> Result<(StatusCode, Json<ResponseUser>), AppError> {

    // 1. extract username from req_user
    let username = req_user.username;
    let hash_password = hash_password(&req_user.password)?;
    let email = if let Some(email) = req_user.email {
        email
    } else {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Need email to create user."));
    };

    // 2. make INSERT INTO db query
    let row = sqlx::query(
                        r#"
                        INSERT INTO users (username, password, email)
                        VALUES ($1, $2, $3)
                        RETURNING id, username, email, created_at
                        "#
                    )
                    .bind(&username)
                    .bind(hash_password)
                    .bind(&email)
                    .fetch_one(&db) // returns exactly one PgRow
                    .await
                    .map_err(|err| {
                        eprintln!("Error inserting into table `users`: {:?}", err);
                        if let Some(err) = err.as_database_error() {
                            if err.message() == "duplicate key value violates unique constraint \"users_username_key\"" {
                                return AppError::new(StatusCode::BAD_REQUEST, "Username already exists!");
                            }
                        }
                        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user")
                    })?;
    
    let resp_user = ResponseUser {
        id: row.get("id"),
        username,
        email,
        token: None
    };

    Ok(
        (
            StatusCode::CREATED, 
            Json(resp_user)
        )
    )
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, 4)
        .map_err(|err| {
            eprintln!("Error hashing password: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Error securing password"
            )
        })
}

pub fn verify_password(password: &str, hash_pass: &str) -> Result<bool, AppError> {
    println!("in verify pass");
    verify(password, hash_pass)
        .map_err(|err| {
            eprintln!("User provided invalid password: {:?}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Error verifying user details.."
            )
        })
}