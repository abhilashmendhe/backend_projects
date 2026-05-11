use actix_web::{HttpResponse, http::StatusCode, web};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use crate::{
    models::users_model::UserModel,
    utils::{
        app_state::AppState,
        errors::{AppError, ExpenseTrackerErr},
    },
};

#[derive(Debug, Deserialize)]
pub struct UpdateExpenseRequest {
    pub amount: BigDecimal,
    pub description: String,
    pub expense_date: NaiveDate,
}

pub async fn update_expense(
    user: UserModel,
    id: web::Path<i32>,
    expense_req: web::Json<UpdateExpenseRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    let id = id.into_inner();

    // 1. Check if responses are null
    let description = expense_req.description.trim();
    if description.eq("") {
        return Err(ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Category name or description can't be empty!",
        )));
    }

    let expense_date = DateTime::<Utc>::from_naive_utc_and_offset(
        expense_req.expense_date.and_hms_opt(0, 0, 0).unwrap(),
        Utc,
    );

    sqlx::query(
        r#"
            UPDATE expenses SET description=$1, amount=$2, expense_date=$3 WHERE id=$4 and user_id=$5
        "#,
    )
    .bind(description)
    .bind(expense_req.amount.clone())
    .bind(expense_date)
    .bind(id)
    .bind(user.id)
    .execute(&app_state.pool)
    .await
    .map_err(|_err| {
        println!("{:?}",_err);
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update table",
        ))
    })?;

    Ok(HttpResponse::Ok().body("Updated expense".to_string()))
}
