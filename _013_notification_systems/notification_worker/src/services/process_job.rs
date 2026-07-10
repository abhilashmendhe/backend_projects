use chrono::{DateTime, Utc};
use redis::{AsyncCommands, aio::MultiplexedConnection, streams::StreamId};
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    services::{
        create_notification_deliverables::create_notification_deliverables,
        gateway_request::{NotifyRequest, make_gateway_request},
    },
    utils::error::NotificationWorkerErr,
};

#[derive(Debug, Deserialize)]
pub struct NotificationDetails {
    pub title: String,
    pub body: Option<String>,
}

// #[derive(Debug, Deserialize)]
// struct SuccessResponse {
//     message_id: String,
//     status: String,
// }

// #[derive(Debug, Deserialize)]
// struct ErrorResponse {
//     error: String,
// }

pub async fn process_job(
    priority: u8,
    max_retry_count: u8,
    platform: String,
    r_stream_group_name: String,
    job: StreamId,
    url_gateway: String,
    callback_url: String,
    db_conn: PgPool,
    q_conn: &mut MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    // 0. store values
    let mut n_id = -1;
    let mut device_id = -1;
    // 1. create notify req struct
    let mut notify_req = NotifyRequest::new();
    notify_req.platform = platform.to_string();
    notify_req.callback_url = callback_url.to_string();
    notify_req.priority = if priority == 0 {
        "high".to_string()
    } else {
        "low".to_string()
    };

    // 2. get value and store in notify req
    for (key, value) in job.map {
        match value {
            redis::Value::BulkString(items) => {
                let s_value = String::from_utf8_lossy(&items).to_string();
                // println!("{}  -   {}",key, s_value);
                if key.eq("notification_id") {
                    n_id = s_value.parse::<i64>()?;
                } else if key.eq("event_id") {
                    notify_req.event_id = s_value;
                } else if key.eq("device_id") {
                    // fetch device token from devices from device id
                    device_id = s_value.parse::<i64>()?;
                    let device_token = sqlx::query_scalar!(
                        r#"
                        SELECT device_token FROM devices WHERE id = $1
                        "#,
                        device_id
                    )
                    .fetch_one(&db_conn)
                    .await?;
                    notify_req.device_token = device_token;
                }
            }
            _ => todo!(),
        }
    }
    // 2. fetch title and body from notifcation table
    let notification_details = sqlx::query_as!(
        NotificationDetails,
        r#"
            SELECT title, body FROM notifications WHERE id = $1
        "#,
        n_id
    )
    .fetch_one(&db_conn)
    .await?;
    // println!("{:?}", db_result);
    notify_req.title = notification_details.title;
    notify_req.body = if notification_details.body == None {
        "".to_string()
    } else {
        notification_details.body.unwrap()
    };

    // 3. make request to the gateway and get response
    // println!("{:?}", notify_req);
    let response = make_gateway_request(url_gateway, notify_req).await?;
    // println!("{} - {}", response.status(), job.id);
    // println!("{:?}", response.status());
    // println!("{:?}", response.headers());
    // println!("{:?}", response.text().await);
    // 4. From the gateway response, insert values in notification_deliverables table

    // 5. handle the statuses in respect to codes
    if response.status() == StatusCode::ACCEPTED {
        // insert/update notification_deliverables table
        create_notification_deliverables(n_id, device_id, 0, 0, db_conn.clone()).await?;
        // xack
        let _ = q_conn
            .xack::<String, String, String, String>(
                format!("{}-{}", platform, priority),
                r_stream_group_name,
                &[job.id],
            )
            .await?;
    } else if response.status() == StatusCode::GONE {
        // insert/update notification_deliverables table
        create_notification_deliverables(n_id, device_id, 1, 0, db_conn.clone()).await?;
        // update devices table and set it to false
        sqlx::query!(
            r#"
            UPDATE devices SET is_active = $1 WHERE id = $2
            "#,
            false,
            device_id
        )
        .execute(&db_conn)
        .await?;
        // xack
        let _ = q_conn
            .xack::<String, String, String, String>(
                format!("{}-{}", platform, priority),
                r_stream_group_name,
                &[job.id],
            )
            .await?;
    } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
        // retry after a few seconds. extract seconds from response.headers
        // also update the retry count + 1 in the notification_deliverables table

        // if retry count (respect to priority) reaches limit, then set them as failed
        // and create an entry in notification_logs table
    } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR
        || response.status() == StatusCode::BAD_GATEWAY
    {
        // exponential retry

        // if retry count (respect to priority) reaches limit, then set them as failed
        // and create an entry in notification_logs table
    }

    // println!();
    Ok(())
}
