use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use sqlx::Row;

use crate::routes::users::create_user::verify_password;
use crate::routes::users::ResponseUser;
use crate::utils::config::Config;
use crate::utils::jwt::create_token;
use crate::{routes::users::RequestUser, utils::errors::AppError};

pub async fn login(
    State(db): State<PgPool>,
    State(config): State<Config>,
    Json(req_user): Json<RequestUser>
) -> Result<Json<ResponseUser>, AppError> {
    let username = req_user.username;
    let password = req_user.password;

    // 1. fetch username, and hash_password for the speciied username and password
    let user_row = sqlx::query(r#"
        SELECT id, username, password, email FROM users WHERE username=$1
    "#)
    .bind(&username)
    .fetch_one(&db)
    .await
    .map_err(|err| {
        eprintln!("Error fetching username: {:?}", err);
        if let Some(_) = err.as_database_error() {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error fetching user")
        } else {
            AppError::new(StatusCode::BAD_REQUEST, "Incorrect username and/or password")
        }
    })?;

    // 2. get hash password and then verify if it's a valid password
    let hash_password: String = user_row.get("password");
    
    if !verify_password(&password, &hash_password)? {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Incorrect username and/or password"));
    }

    // 3. Now create JWT token
    let jwt_secret = config.jwt_secret();
    let token = create_token(&jwt_secret, username.clone()).await?;

    // 4. now save the token into database
    sqlx::query(r#"
        UPDATE users SET token=$1 WHERE username=$2
        "#)
        .bind(&token)
        .bind(&username)
        .execute(&db)
        .await
        .map_err(|err| {
            eprintln!("Error updating token for the user: {:?}",err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error login!")
        })?;
    
    let resp_user = ResponseUser {
        id: user_row.get("id"),
        username,
        email: user_row.get("email"),
        token: Some(token),
    };

    Ok(Json(resp_user))
}