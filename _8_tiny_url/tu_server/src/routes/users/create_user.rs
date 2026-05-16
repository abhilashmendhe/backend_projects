use actix_web::{HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    models::user_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
        hash_verify_pass::hash_password,
    },
};

#[derive(Deserialize)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub token: Option<String>,
}

pub async fn create_user(
    req_user: web::Json<RequestUser>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    // 1. Check if username, password, and email is not empty
    let username = &req_user.username;
    let password = &req_user.password;
    let email = &req_user.email;

    if username.eq("") || password.eq("") || email.eq("") {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "username, password, and/or email cannot be empty",
        )));
    }

    // 2. Create a hash password
    let hash_password = hash_password(password)?;

    // 3. Create user using insert query
    let resp_user = sqlx::query_as!(
        ResponseUser,
        r#"
            INSERT INTO users(username, password, email) VALUES($1,$2,$3)
            RETURNING id, created_at as "created_at!", token
        "#,
        username,
        hash_password,
        email
    )
    .fetch_one(app_data.pool())
    .await
    .map_err(|err| {
        tracing::error!("Error creating a user: {:?}", err);
        if let Some(pg_err) = err.as_database_error() {
            if let Some(e_code) = pg_err.code() {
                if e_code.to_string().eq("23505") {
                    return TinyUrlError::AppError(AppError::new(
                        StatusCode::BAD_REQUEST,
                        "user already exists!",
                    ));
                }
            }
        }
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create user!",
        ))
    })?;

    let user = UserModel {
        id: resp_user.id,
        username: username.clone(),
        password: password.clone(),
        email: email.clone(),
        created_at: Some(resp_user.created_at),
        deleted_at: None,
        token: None,
    };
    Ok(HttpResponse::Created().json(user))
}
