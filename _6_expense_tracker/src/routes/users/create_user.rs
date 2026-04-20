use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{routes::users::{RequestUser, ResponseUser}, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}, hash_helper::hash_password}};

pub async fn create_user(
    user_req: web::Json<RequestUser>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, ExpenseTrackerErr> {

    // 1. Check
    let username = user_req.username.trim();
    let password = user_req.password.trim();
    let email = user_req.email.trim();
    if username.eq("") || password.eq("") || email.eq("") {
        return Err(ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Username, password and/or email cannot be empty.\nPlease try again later."
        )))
    }


    // 2. Create hashed password
    let hash_password = hash_password(password)?;
    let user = sqlx::query_as!(
                ResponseUser,
                r#"
                INSERT INTO users (username, password, email)
                VALUES ($1, $2, $3) 
                RETURNING id, username, email, created_at as "created_at!", token
                "#,
                username,
                hash_password,
                email
            )
            .fetch_one(&app_state.pool)
            .await
            .map_err(|err|{
                tracing::error!("Error inserting into table `users`: {:?}", err);
                if let Some(err) = err.as_database_error() {
                    if err.message() == "duplicate key value violates unique constraint \"users_username_key\"" {
                        return ExpenseTrackerErr::AppError(AppError::new(StatusCode::BAD_REQUEST, "Username already exists!"));
                    }
                }
                ExpenseTrackerErr::AppError(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))
            })?;
    Ok(HttpResponse::Created().json(user))
}