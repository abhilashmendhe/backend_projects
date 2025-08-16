use serde::Deserialize;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use sqlx::Row;

use crate::routes::users::create_user::hash_password;
use crate::routes::users::create_user::verify_password;
use crate::utils::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct RequestUpdatePassUser {
    pub username: String, 
    pub old_password: String,
    pub new_password: String
}

pub async fn update_user_password(
    State(db): State<PgPool>,
    Json(req_update_pass): Json<RequestUpdatePassUser>
) -> Result<(StatusCode, String), AppError> {

    let username = req_update_pass.username;
    let old_pass = req_update_pass.old_password;
    let new_pass = req_update_pass.new_password;

    // 1. Fetch user to update the passowrd
    let user_row = sqlx::query(r#"
        SELECT * FROM users WHERE username=$1
    "#)
    .bind(&username)
    .fetch_one(&db)
    .await
    .map_err(|err|{
        eprintln!("Error updating password: {:?}",err);
        AppError::new(
            StatusCode::BAD_REQUEST, 
            "Username and/or password incorrect!"
        )
    })?;

    // 2. verify if the old password matches
    let old_hash_pass = user_row.get::<String,_>("password");
    
    if !verify_password(&old_pass, &old_hash_pass)? {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username and/or password incorrect!"
        ))
    }

    // 3. create new hash pass
    let new_hash_pass = hash_password(&new_pass)?;

    // 4. update query to update to new hash pass
    let _ = sqlx::query(r#"
        UPDATE users SET password=$1 WHERE username=$2
    "#)
    .bind(&new_hash_pass)
    .bind(username)
    .execute(&db)
    .await
    .map_err(|err|{
        eprintln!("Error updating password: {:?}",err);
        AppError::new(
            StatusCode::BAD_REQUEST, 
            "Username and/or password incorrect!"
        )
    })?;
    Ok((StatusCode::CREATED, "User password update successful!".to_string()))
}