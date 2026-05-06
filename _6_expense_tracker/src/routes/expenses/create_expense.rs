use actix_web::{HttpResponse, http::StatusCode, web};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

use crate::{models::users_model::UserModel, utils::{app_state::AppState, errors::{AppError, ExpenseTrackerErr}}};

#[derive(Debug, Deserialize)]
pub struct ExpenseRequest {
    pub category_name: String,
    pub amount: BigDecimal,
    pub description: String,
    pub expense_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct CategoryId {
    pub id: i32
}

#[derive(Debug, Serialize)]
pub struct ExpenseExpectedResponse {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize)]
pub struct ExpenseResponse {
    id: i32,
    user_id: i32,  
    category_id: i32, 
    amount: BigDecimal, 
    description: String, 
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    expense_date: NaiveDate
}

pub async fn create_expense(
    user: UserModel,
    expense_req: web::Json<ExpenseRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {

    // 1. Check if responses are null
    let category_name  = expense_req.category_name.trim();
    let description = expense_req.description.trim();
    if category_name.eq("") || description.eq("") {
        return Err(ExpenseTrackerErr::AppError(
            AppError::new(
                StatusCode::BAD_REQUEST, 
                "Category name or description can't be empty!")
        ));
    }

    // 1. make insertion in categories table
    let c = sqlx::query_as!(
        CategoryId,
        r#"
            INSERT INTO categories(name, user_id) VALUES ($1, $2)
            RETURNING id
        "#,
        category_name,
        user.id
    ).fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        tracing::error!("Error inserting into table `categories`: {:?}", err);
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create a row in categories",
        ))
    })?;

    let expense_date = DateTime::<Utc>::from_naive_utc_and_offset(
    expense_req.expense_date.and_hms_opt(0, 0, 0).unwrap(),Utc);

    // expense_req.expense_date.
    // 2. make insertion in expense table
    let expected_resp = sqlx::query_as!(
        ExpenseExpectedResponse,
        r#"
        INSERT INTO expenses (user_id,category_id,amount,description,expense_date)
        VALUES ($1,$2,$3,$4,$5)
        RETURNING id, created_at as "created_at!", updated_at as "updated_at!"
        "#,
        user.id,
        c.id,
        expense_req.amount,
        expense_req.description.clone(),
        expense_date
    ).fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        tracing::error!("Error inserting into table `expenses`: {:?}", err);
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create a row in expenses",
        ))
    })?;

    let exp_resp = ExpenseResponse {
        id: expected_resp.id,
        user_id: user.id,
        category_id: c.id,
        amount: expense_req.amount.clone(),
        description: expense_req.description.clone(),
        created_at: expected_resp.created_at,
        updated_at: expected_resp.updated_at,
        expense_date: expense_req.expense_date,
    };
    Ok(HttpResponse::Ok().json(exp_resp))
    // Ok(HttpResponse::Ok().body("body"))
}