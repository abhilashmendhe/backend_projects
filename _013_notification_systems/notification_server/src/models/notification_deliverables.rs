use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationDeliverables {
    pub id: i64,
    pub notification_id: i64,
    pub device_id: i64,
    pub status: i16,
    pub retry_count: i16,
    pub next_retry_at: Option<DateTime<Utc>>,
}
