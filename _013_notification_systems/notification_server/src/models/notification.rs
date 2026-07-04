use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub event_id: String,
    pub title: String,
    pub body: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub priority: i16,
    pub created_at: Option<DateTime<Utc>>,
}
