use chrono::{TimeDelta, Utc};
use redis::{AsyncCommands, aio::MultiplexedConnection, streams::StreamId};
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    services::{
        create_notification_deliverables::create_notification_deliverables,
        gateway_request::{NotifyRequest, make_gateway_request},
        get_join_notif_deliv::get_join_notification_deliverables,
        retry_mechanism::retry_mechanims,
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
    let mut stream_data = vec![];

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
                stream_data.push((key.clone(), s_value.clone()));
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

    // 2. make select query for notifications with left join notifications_deliverables
    let notification_details = get_join_notification_deliverables(n_id, db_conn.clone()).await?;
    notify_req.title = notification_details.no_title;
    notify_req.body = if notification_details.no_body == None {
        "".to_string()
    } else {
        notification_details.no_body.unwrap()
    };

    // 3. check if retry time, if current is less than the retry then return
    let now = Utc::now();
    if let Some(retry_time) = notification_details.nod_next_retry_at {
        if now < retry_time {
            /*
            // re-add
            q_conn
                .xadd::<String, String, String, String>(
                    stream.to_string(),
                    "*".to_string(),
                    &stream_data,
                )
                .await?;

            // remove
            q_conn.xdel::<String, String>(stream, &[job_id]).await?;
            */
            return Ok(());
        }
    }

    // 4. make request to the gateway and get response
    println!("{:?}", notify_req);
    let response = make_gateway_request(url_gateway, notify_req.clone()).await?;
    println!("{} - {}", response.status(), job.id);
    // println!("{:?}", response.status());
    // println!("{:?}", response.headers());
    // println!("{:?}", response.text().await);

    // 5. handle the statuses in respect to codes
    if response.status() == StatusCode::ACCEPTED {
        // insert/update notification_deliverables table
        if notification_details.nod_id.is_none() {
            create_notification_deliverables(n_id, device_id, 0, 0, None, db_conn.clone()).await?;
        } else {
            if notification_details.nod_status != Some(0) {
                sqlx::query!(
                    r#"
                        UPDATE notification_deliverables SET status = 0 WHERE id = $1
                    "#,
                    notification_details.nod_id
                )
                .execute(&db_conn)
                .await?;
            }
        }
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
        if notification_details.nod_id.is_none() {
            create_notification_deliverables(n_id, device_id, 1, 0, None, db_conn.clone()).await?;
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
        }
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
        let hv = response.headers().get("Retry-After").unwrap();
        let retr_val_s = hv.to_str()?;
        let retr_val_n = retr_val_s.parse::<i64>()?;
        let retry_at = now + TimeDelta::seconds(retr_val_n);
        if notification_details.nod_id.is_none() {
            create_notification_deliverables(
                n_id,
                device_id,
                2,
                1,
                Some(retry_at),
                db_conn.clone(),
            )
            .await?;
        } else {
            let retry_count = notification_details.nod_retry_count.unwrap();
            let nod_id = notification_details.nod_id.unwrap();
            retry_mechanims(
                nod_id,
                retry_count,
                max_retry_count as i16,
                retry_at,
                notify_req.event_id,
                "".to_string(),
                db_conn,
            )
            .await?;
        }
    } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR
        || response.status() == StatusCode::BAD_GATEWAY
    {
        if notification_details.nod_id.is_none() {
            let ret_now = now + TimeDelta::seconds(1);
            create_notification_deliverables(n_id, device_id, 2, 1, Some(ret_now), db_conn.clone())
                .await?;
        } else {
            let retry_count = notification_details.nod_retry_count.unwrap();
            let retr_val_n = 2_i64.pow(retry_count as u32);
            let retry_at = now + TimeDelta::seconds(retr_val_n);
            let nod_id = notification_details.nod_id.unwrap();
            retry_mechanims(
                nod_id,
                retry_count,
                max_retry_count as i16,
                retry_at,
                notify_req.event_id,
                "".to_string(),
                db_conn,
            )
            .await?;
        }
    }
    Ok(())
}
