use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: i64,
    pub user_id: i64,
    pub device_token: String,
    pub platform: String,
    pub created_at: Option<DateTime<Utc>>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}