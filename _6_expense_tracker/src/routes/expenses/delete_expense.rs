use actix_web::{HttpResponse, http::StatusCode, web};

use crate::{
    models::{expenses_models::ExpenseResponseModel, users_model::UserModel},
    utils::{
        app_state::AppState,
        errors::{AppError, ExpenseTrackerErr},
    },
};

pub async fn delete_expense(
    user: UserModel,
    id: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    let id = id.into_inner();
    let deleted_expense = sqlx::query_as!(
        ExpenseResponseModel,
        r#"
        DELETE FROM expenses
        WHERE id = $1 AND user_id = $2
        RETURNING
            id,
            user_id,
            category_id,
            amount,
            description,
            created_at,
            updated_at,
            expense_date
        "#,
        id,
        user.id
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|_err| {
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete expense with id: {}", id),
        ))
    })?;

    Ok(HttpResponse::Ok().json(deleted_expense))
}
