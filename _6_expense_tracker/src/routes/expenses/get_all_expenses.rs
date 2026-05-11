use std::collections::HashMap;

use actix_web::{HttpResponse, http::StatusCode, web};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::{
    models::{
        expenses_models::{Category, ExpenseResponseModel},
        users_model::UserModel,
    },
    utils::{
        app_state::AppState,
        errors::{AppError, ExpenseTrackerErr},
    },
};

/*
    Make GET /expenses powerful:
      1. Filter by date range
      2. Filter by category
      3. Filter by amount range
      4. Search by text (note/description)
      5. Pagination + sorting
*/

#[derive(Debug, Deserialize, Clone)]
pub struct ExpenseQuery {
    // date range
    start_date: Option<String>,
    end_date: Option<String>,

    // category
    category: Option<String>,

    // amount range
    min_amt: Option<f64>,
    max_amt: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct FullExpenseResponseModel {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub amount: BigDecimal,

    pub description: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expense_date: Option<DateTime<Utc>>,
    pub category: Option<String>,
}

pub async fn get_all_expenses(
    user: UserModel,
    query_p: web::Query<ExpenseQuery>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, ExpenseTrackerErr> {
    // println!("{:?}", query_p);
    let mut qb = filter_query(user.id, &query_p)?;
    let expenses = qb
        .build_query_as::<ExpenseResponseModel>()
        .fetch_all(&app_state.pool)
        .await
        .map_err(|_err| {
            ExpenseTrackerErr::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch data from `expenses`".to_string(),
            ))
        })?;

    let categories = sqlx::query_as!(
        Category,
        r#"
        SELECT * FROM categories WHERE user_id=$1
        "#,
        user.id
    )
    .fetch_all(&app_state.pool)
    .await
    .map_err(|err| {
        ExpenseTrackerErr::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch data from `categories` table".to_string(),
        ))
    })?;

    let map: HashMap<i32, String> = categories
        .into_iter()
        .filter_map(|category| category.id.map(|cid| (cid, category.name.unwrap())))
        .collect();

    let mut full_expenses = vec![];

    if let Some(category) = &query_p.category {
        for exp in expenses {
            if category.eq(map.get(&exp.category_id).unwrap()) {
                full_expenses.push(FullExpenseResponseModel {
                    id: exp.id,
                    user_id: exp.user_id,
                    category_id: exp.category_id,
                    amount: exp.amount,
                    description: exp.description,
                    created_at: exp.created_at,
                    updated_at: exp.updated_at,
                    expense_date: exp.expense_date,
                    category: map.get(&exp.category_id).cloned(),
                });
            }
        }
    } else {
        for exp in expenses {
            full_expenses.push(FullExpenseResponseModel {
                id: exp.id,
                user_id: exp.user_id,
                category_id: exp.category_id,
                amount: exp.amount,
                description: exp.description,
                created_at: exp.created_at,
                updated_at: exp.updated_at,
                expense_date: exp.expense_date,
                category: map.get(&exp.category_id).cloned(),
            });
        }
    }

    Ok(HttpResponse::Ok().json(full_expenses))
    // Ok(HttpResponse::Ok().body("body".to_string()))
}

pub fn filter_query(
    user_id: i32,
    query_p: &web::Query<ExpenseQuery>,
) -> Result<QueryBuilder<'_, Postgres>, ExpenseTrackerErr> {
    // println!("quries: {:?}", query_p);
    let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM expenses WHERE user_id = ");

    qb.push_bind(user_id);

    // date range
    if let Some(start) = &query_p.start_date {
        qb.push(" AND expense_date >= ");
        let start_dt = DateTime::parse_from_rfc3339(start)
            .map_err(|err| {
                ExpenseTrackerErr::AppError(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to convert String date to Datetime<Utc> in filter_query fn".to_string(),
                ))
            })?
            .with_timezone(&Utc);
        qb.push_bind(start_dt);
    }

    if let Some(end) = &query_p.end_date {
        qb.push(" AND expense_date <= ");
        let end_dt = DateTime::parse_from_rfc3339(end)
            .map_err(|err| {
                ExpenseTrackerErr::AppError(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to convert String date to Datetime<Utc> in filter_query fn".to_string(),
                ))
            })?
            .with_timezone(&Utc);
        qb.push_bind(end_dt);
    }

    // amount range
    if let Some(min) = query_p.min_amt {
        qb.push(" AND amount >= ");
        qb.push_bind(min);
    }

    if let Some(max) = query_p.max_amt {
        qb.push(" AND amount <= ");
        qb.push_bind(max);
    }

    Ok(qb)
}
