use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserDB {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub token: Option<String>,
    pub email: String
}