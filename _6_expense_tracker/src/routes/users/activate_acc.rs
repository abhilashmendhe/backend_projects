use actix_web::{HttpResponse, http::StatusCode, web};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}, hash_helper::verify_hash_password};

#[derive(Debug, Deserialize)]
pub struct ActivateAccount {
    username: String, 
    password: String
}

#[derive(Debug, Serialize)]
pub struct UserActivate {
    pub id: i32, 
    username: String, 
    email: String,
}

#[derive(Debug,Serialize)]
pub struct UserActivateResponse {
    useractivate: UserActivate,
    message: String
}

pub async fn account_activate(
    user_req: web::Json<ActivateAccount>, 
    app_state: web::Data<AppState>
) -> Result<HttpResponse, ExpenseTrackerErr> {

    // 1. Check if username passed correctly
    let username = user_req.username.trim();
    let password = user_req.password.trim();
    if username.eq("") || password.eq("") {
        return Err(
            ExpenseTrackerErr::AppError(AppError::new(StatusCode::BAD_REQUEST, "Username cannot be null."))
        );
    }

    // 2. Verify password
    let user = sqlx::query(
        r#"SELECT password from users WHERE username=$1"#
    ).bind(username)
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        if err.to_string() == "RowNotFound".to_string() {
            return ExpenseTrackerErr::AppError(AppError::new(StatusCode::BAD_REQUEST, "No username found!"));
        }
        ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error activating account!"))
    })?;
    
    let hashed_password: String = user.get(0);
    verify_hash_password(password, &hashed_password)?;
    
    // 3. Run UPDATE query
    let user = sqlx::query_as!(UserActivate,
        r#"
        UPDATE users SET deleted_at=NULL,created_at=Now(),token=NULL where username=$1
        RETURNING id, username, email
        "#,
        username
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        if err.to_string() == "RowNotFound".to_string() {
            return ExpenseTrackerErr::AppError(AppError::new(StatusCode::BAD_REQUEST, "No username found!"));
        }
        ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error activating account!"))
    })?;

    let user_activate_resp = UserActivateResponse {
        useractivate: user,
        message: "Account activation successful!".to_string()
    };
    Ok(HttpResponse::Created().json(user_activate_resp))
}