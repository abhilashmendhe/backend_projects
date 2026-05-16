use actix_web::{HttpResponse, http::StatusCode, web};
use serde::{Deserialize, Serialize};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
        hash_verify_pass::verify_hash_password,
        jwt::{create_token, validate_token},
    },
};

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginUserResponse {
    pub id: i64,
    pub username: String,
    pub token: String,
}

pub async fn login(
    login_user_req: web::Json<LoginUserRequest>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    // 1. Check if username, and password is not empty
    let username = &login_user_req.username;
    let password = &login_user_req.password;
    if username.eq("") || password.eq("") {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "username and/or password cannot be empty!",
        )));
    }

    // 2. Fetch user from DB
    let user = sqlx::query_as!(
        UserModel,
        r#"
            SELECT * FROM users WHERE username=$1
        "#,
        username,
    )
    .fetch_one(app_data.pool())
    .await
    .map_err(|err| {
        tracing::error!("Error fetching user: {:?}", err);
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch user!",
        ))
    })?;

    // 2.5 Check if user delted
    // 1. check if already user delted
    if let Some(_) = user.deleted_at {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "User doesn't exists or deleted!",
        )));
    }

    // 3. Now verify hashed password
    verify_hash_password(password, &user.password)?;
    // println!("Verified ");
    // 3.5 If JWT already present, return the user
    if let Some(token_value) = user.token {
        let f = match validate_token(&app_data.config().jwt_secret(), &token_value) {
            Ok(flag) => flag,
            Err(_) => false,
        };
        if f {
            let login_resp_user = LoginUserResponse {
                id: user.id,
                username: username.to_string(),
                token: token_value.to_string(),
            };
            return Ok(HttpResponse::Ok().json(login_resp_user));
        }
    }

    // 4. Create a JWT token
    let jwt_token = create_token(&app_data.config().jwt_secret(), username.to_string())?;

    // 5. Now update the user row, and set the JWT token value
    sqlx::query!(r#"UPDATE users SET token=$1"#, jwt_token)
        .execute(app_data.pool())
        .await
        .map_err(|err| {
            tracing::error!("Error updating user: {:?}", err);
            TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update user with JWT!",
            ))
        })?;

    let login_resp_user = LoginUserResponse {
        id: user.id,
        username: username.to_string(),
        token: jwt_token,
    };
    Ok(HttpResponse::Ok().json(login_resp_user))
}
