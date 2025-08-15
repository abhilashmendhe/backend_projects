use axum::{extract::State, http::StatusCode, Extension};
use sqlx::PgPool;

use crate::{models::user_model::UserDB, utils::errors::AppError};

pub async fn logout(
    Extension(user): Extension<UserDB>,
    State(db): State<PgPool>
) -> Result<StatusCode, AppError> {

    sqlx::query(r#"UPDATE users SET token=$1 WHERE id=$2"#)
        .bind("")
        .bind(user.id)
        .execute(&db)
        .await
        .map_err(|err| {
            eprint!("Error user logout: {:?}",err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was problem logging out..."
            )
        })?;
    Ok(StatusCode::OK)
}