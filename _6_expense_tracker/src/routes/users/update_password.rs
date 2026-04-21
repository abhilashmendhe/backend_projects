use actix_web::{HttpResponse, http::StatusCode, web};
use serde::Deserialize;
use crate::{models::users_model::UserModel, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}, hash_helper::hash_password}};

#[derive(Debug, Deserialize)]
pub struct UpdatePassReq {
    password: String
}

pub async fn update_password(
    user: UserModel,
    pass_req: web::Json<UpdatePassReq>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    
    // 1. create hash pass 
    let hash_password = hash_password(&pass_req.password)?;
    sqlx::query(r#"UPDATE users SET password=$1 WHERE username=$2 AND deleted_at IS NULL"#)
        .bind(hash_password)
        .bind(&user.username)
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
    Ok(HttpResponse::Ok().body("Updated Password"))
}