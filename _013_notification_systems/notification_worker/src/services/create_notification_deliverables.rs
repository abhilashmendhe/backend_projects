use sqlx::PgPool;

use crate::utils::error::NotificationWorkerErr;

pub async fn create_notification_deliverables(
    notification_id: i64,
    device_id: i64,
    status: i16,
    retry_count: i16,
    db_pool: PgPool,
) -> Result<(), NotificationWorkerErr> {
    sqlx::query!(
        r#"
        INSERT INTO notification_deliverables (notification_id, device_id, status, retry_count) 
        VALUES 
        ($1, $2, $3, $4)
        "#,
        notification_id,
        device_id,
        status,
        retry_count
    )
    .execute(&db_pool)
    .await?;
    Ok(())
}
