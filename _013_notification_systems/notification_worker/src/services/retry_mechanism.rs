use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::utils::error::NotificationWorkerErr;

pub async fn retry_mechanims(
    nod_id: i64,
    db_retry_count: i16,
    max_retry_count: i16,
    retry_at: DateTime<Utc>,
    event_id: String,
    _message: String,
    db_conn: PgPool,
) -> Result<(), NotificationWorkerErr> {
    if db_retry_count > max_retry_count {
        // if retry count (respect to priority) reaches limit, then set notification_deliverables them as failed
        sqlx::query!(
            r#"
                UPDATE notification_deliverables SET status = 1 WHERE id = $1
            "#,
            nod_id
        )
        .execute(&db_conn)
        .await?;
        // create an entry in notification_logs table
        let message = "Failed to deliver".to_string();
        sqlx::query!(
            r#"
                INSERT INTO notification_logs(delivery_id, event_id, message) VALUES($1, $2, $3) 
            "#,
            nod_id,
            event_id,
            message
        )
        .execute(&db_conn)
        .await?;
    } else {
        // update notification_deliverables table
        sqlx::query!(
            r#"
                UPDATE notification_deliverables SET retry_count = $1, next_retry_at = $2 
                WHERE id = $3
            "#,
            db_retry_count + 1,
            retry_at,
            nod_id
        )
        .execute(&db_conn)
        .await?;
    }
    Ok(())
}
