use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::{expenses_models::ExpenseResponseModel, users_model::UserModel},
    utils::{
        app_state::AppState,
        errors::{AppError, ExpenseTrackerErr},
    },
};

pub async fn get_all_expenses(
    user: UserModel,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    let expenses = sqlx::query_as!(
        ExpenseResponseModel,
        r#"
        SELECT * FROM expenses WHERE user_id=$1
    "#,
        user.id
    )
    // .bind(user.id)
    .fetch_all(&app_state.pool)
    .await
    .map_err(|err| {
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Expenses not found!".to_string(),
        ))
    })?;
    Ok(HttpResponse::Ok().json(expenses))
    // Ok(HttpResponse::Ok().body("body".to_string()))
}
