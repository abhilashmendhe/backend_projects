use redis::{AsyncTypedCommands, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// --------------------- Notification outbox, notiications and deivces combined data model --------------------
#[derive(Debug, Deserialize)]
pub struct NotificationDeviceOutbox {
    pub no_id: i64,
    pub no_n_id: i64,
    pub no_user_id: i64,
    pub n_event_id: String,
    pub n_priority: i16,
    pub d_id: i64,
    pub d_device_token: String,
    pub d_platform: String,
}

// --------------------- Message model to enqueue in jobs queue --------------------
#[derive(Debug, Serialize)]
pub struct Message {
    pub notification_outbox_id: i64,
    pub notification_id: i64,
    pub event_id: String,
    pub device_token: String,
}

pub async fn push_to_redis_queue(
    db_pool: PgPool,
    redis_ios_q: MultiplexedConnection,
    redis_android_q: MultiplexedConnection,
    background_fetch_limit_rows: i64,
) {
    match sqlx::query_as!(
        NotificationDeviceOutbox,
        r#"
        SELECT
            no.id AS no_id,
            no.notification_id AS no_n_id,
            no.user_id AS no_user_id,
            n.event_id AS n_event_id,
            n.priority AS n_priority,
            d.id AS d_id,
            d.device_token AS d_device_token,
            d.platform AS d_platform
        FROM notification_outbox AS no
        INNER JOIN notifications AS n on no.notification_id = n.id
        INNER JOIN devices AS d on no.user_id = d.user_id WHERE no.published=False AND d.is_active=True LIMIT $1;
        "#,
        background_fetch_limit_rows
    ).fetch_all(&db_pool)
    .await {
        Ok(noti_device_outbox) => {
            let mut no_outbox_ids: Vec<i64> = vec![];
            // 1. enqueue in queue 
            for ndo in noti_device_outbox {
                let mut q_conn = if ndo.d_platform.eq("ios") {
                    redis_ios_q.clone()
                } else if ndo.d_platform.eq("android") {
                    redis_android_q.clone()
                } else {
                    // Implement queues for sms, emails etc.
                    continue
                };
                match serde_json::to_string(&Message {
                    notification_outbox_id: ndo.no_id,
                    notification_id: ndo.no_n_id,
                    event_id: ndo.n_event_id.to_string(),
                    device_token: ndo.d_device_token.to_string(),
                }) {
                    Ok(message) => {
                        match q_conn.lpush::<String, String>(ndo.n_priority.to_string(), message).await {
                            Ok(_) => {
                                no_outbox_ids.push(ndo.no_id);
                            },
                            Err(_) => { continue },
                        }
                    },
                    Err(_) => {
                        continue
                    },
                }
            }

            // 2. update notification_oubox table set published to true
            for no_id in no_outbox_ids {
                let _ = sqlx::query!(
                    r#"UPDATE notification_outbox SET published = $1 WHERE id = $2"#,
                    true,
                    no_id
                ).execute(&db_pool)
                .await;
            }
        },
        Err(err) => {
            tracing::error!("->>\t Error fetching details to enqueue jobs in queue!\n{err:?}");
        },
    }
}
