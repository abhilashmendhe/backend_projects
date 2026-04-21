use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{models::users_model::UserModel, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}}};

pub async fn logout(
    user: UserModel,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {

    sqlx::query(r#"UPDATE users SET token=NULL WHERE username=$1"#)
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
    Ok(HttpResponse::Ok().body("Logged Out"))
}