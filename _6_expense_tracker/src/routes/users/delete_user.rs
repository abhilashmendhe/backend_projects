use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{models::users_model::UserModel, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}}};

pub async fn delete_user(
    user: UserModel,
    app_state: web::Data<AppState>,
    req_path: web::Path<i32>
) -> Result<HttpResponse, ExpenseTrackerErr>  {

    let id = req_path.into_inner();
    
    if id != user.id {
        return Err(ExpenseTrackerErr::AppError(AppError::new(StatusCode::NOT_FOUND, "User not found!")));
    }

    // update the deleted_at field
    sqlx::query(r#"UPDATE users SET deleted_at=NOW() WHERE username=$2 and id=$3"#)
        .bind(&user.username)
        .bind(&user.id)
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
    Ok(HttpResponse::Created().body("User deleted!"))
}