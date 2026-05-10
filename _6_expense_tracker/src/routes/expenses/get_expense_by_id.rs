use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::{expenses_models::ExpenseResponseModel, users_model::UserModel},
    utils::{
        app_state::AppState,
        errors::{AppError, ExpenseTrackerErr},
    },
};

pub async fn get_expense_by_id(
    user: UserModel,
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    let id = id.into_inner();
    let expense = sqlx::query_as!(
        ExpenseResponseModel,
        r#"
            SELECT * FROM expenses WHERE user_id=$1 and id=$2
        "#,
        user.id,
        id
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|_err| {
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Expense with id:{} for user id:{} not found!", id, user.id),
        ))
    })?;
    Ok(HttpResponse::Ok().json(expense))
}
