//  id | user_id | short_url_code | long_url | created_at | deleted_at | expired_at

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TinyUrlModel {
    pub id: i64,
    pub user_id: i64,
    pub short_url_code: String,
    pub long_url: String,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
}
