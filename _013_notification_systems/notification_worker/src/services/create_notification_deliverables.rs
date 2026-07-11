use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::utils::error::NotificationWorkerErr;

pub async fn create_notification_deliverables(
    notification_id: i64,
    device_id: i64,
    status: i16,
    retry_count: i16,
    next_retry_at: Option<DateTime<Utc>>,
    db_pool: PgPool,
) -> Result<(), NotificationWorkerErr> {
    sqlx::query!(
        r#"
        INSERT INTO notification_deliverables (notification_id, device_id, status, retry_count, next_retry_at) 
        VALUES 
        ($1, $2, $3, $4, $5)
        "#,
        notification_id,
        device_id,
        status,
        retry_count,
        next_retry_at
    )
    .execute(&db_pool)
    .await?;
    Ok(())
}
