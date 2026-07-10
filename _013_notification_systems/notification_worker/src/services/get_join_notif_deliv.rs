use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::utils::error::NotificationWorkerErr;

#[derive(Debug, Deserialize)]
pub struct NotificationNotificationDeliverables {
    pub no_id: i64,
    pub no_user_id: i64,
    pub no_title: String,
    pub no_body: Option<String>,
    pub no_payload: Option<serde_json::Value>,
    pub nod_id: Option<i64>,
    pub nod_device_id: Option<i64>,
    pub nod_status: Option<i16>,
    pub nod_retry_count: Option<i16>,
    pub nod_next_retry_at: Option<DateTime<Utc>>,
}

pub async fn get_join_notification_deliverables(
    notification_id: i64,
    db_conn: PgPool,
) -> Result<NotificationNotificationDeliverables, NotificationWorkerErr> {
    let result = sqlx::query_as!(
        NotificationNotificationDeliverables,
        r#"
         SELECT
        no.id as no_id,
        no.user_id as no_user_id,
        no.title as no_title,
        no.body as no_body,
        no.payload as no_payload,
        nod.id as nod_id,
        nod.device_id as nod_device_id,
        nod.status as nod_status,
        nod.retry_count as nod_retry_count,
        nod.next_retry_at as nod_next_retry_at
    FROM notifications as no
    LEFT JOIN
    notification_deliverables AS nod
    ON
    no.id = nod.notification_id
    WHERE no.id = $1;
        "#,
        notification_id
    )
    .fetch_one(&db_conn)
    .await?;
    Ok(result)
}
