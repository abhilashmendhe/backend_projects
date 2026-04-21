use actix_web::{HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}, hash_helper::verify_hash_password, jwt::{create_token, validate_token}};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String, 
    password: String, 
}

#[derive(Debug, Deserialize)]
pub struct FetchUserInfo {
    password: String, 
    created_at: DateTime<Utc>,
    token: Option<String>
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    username: String, 
    created_at: DateTime<Utc>,
    token: Option<String>
}

pub async fn login(
    app_state: web::Data<AppState>,
    user_req: web::Json<LoginRequest>
) -> Result<HttpResponse, ExpenseTrackerErr> {

    // 1. Check
    let username = user_req.username.trim();
    let password = user_req.password.trim();
    if username.eq("") || password.eq("") {
        return Err(ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username, password and/or email cannot be empty.\nPlease try again later."
        )))
    }

    // 2. fetch hashed_password from db
    let login_user_resp = sqlx::query_as!(
        FetchUserInfo,
        r#"SELECT password, created_at as "created_at!", token FROM users WHERE deleted_at IS NULL and username=$1"#,
        username
    ).fetch_one(&app_state.pool)
    .await
    .map_err(|err|{
        tracing::error!("Error fetching user: {:?}",err);
        // println!("{:?}",);
        if let None = err.as_database_error() {
            return ExpenseTrackerErr::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!"));
        }
        ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"))
    })?;

    // 2. Verify password
    verify_hash_password(password, &login_user_resp.password)?;

    // 3. if token already present, return existing token
    if let Some(token_value) = login_user_resp.token {
        println!("token already present");
        let f = match validate_token(&app_state.config.secret(), &token_value) {
            Ok(flag) => flag,
            Err(_) => false,
        };
        if f {
            let login_resp = LoginResponse {
                username: username.to_string(),
                created_at: login_user_resp.created_at,
                token: Some(token_value),
            };
            return Ok(HttpResponse::Ok().json(login_resp));
        }
    }
    
    // 4. Generate token and update user in db
    let new_token = create_token(&app_state.config.secret(), username.to_string())?;
    sqlx::query(r#"UPDATE users SET token=$1 WHERE username=$2"#)
        .bind(&new_token)
        .bind(&username)
        .execute(&app_state.pool)
        .await
        .map_err(|err| {
            tracing::error!("Error fetching user: {:?}",err);
            // println!("{:?}",);
            if let None = err.as_database_error() {
                return ExpenseTrackerErr::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!"));
            }
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"))
        })?;    

    let login_resp = LoginResponse {
        username: username.to_string(),
        created_at: login_user_resp.created_at,
        token: Some(new_token),
    };
    Ok(HttpResponse::Ok().json(login_resp))
}