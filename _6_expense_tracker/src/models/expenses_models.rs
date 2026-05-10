use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ExpenseResponseModel {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub amount: BigDecimal,

    pub description: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expense_date: Option<DateTime<Utc>>,
}
