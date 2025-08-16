
use axum::{extract::State, http::StatusCode, Extension};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{models::user_model::UserDB, utils::errors::AppError};
#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub username: String
}

pub async fn delete_user(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>
) -> Result<StatusCode, AppError> {

    let now = Utc::now();
    sqlx::query(r#"
        UPDATE users SET deleted_at=$1, token=$2 WHERE id=$3
    "#)
    .bind(now)
    .bind("")
    .bind(user.id)
    .execute(&db)
    .await
    .map_err(|err| {
        eprintln!("Error deleting user: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error deleting account! Please try again later")
    })?;

    Ok(StatusCode::CREATED)
}