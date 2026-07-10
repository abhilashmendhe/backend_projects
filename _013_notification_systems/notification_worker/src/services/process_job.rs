use redis::{aio::MultiplexedConnection, streams::StreamId};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    services::gateway_request::{NotifyRequest, make_gateway_request},
    utils::error::NotificationWorkerErr,
};

#[derive(Debug, Deserialize)]
pub struct NotificationDetails {
    pub title: String,
    pub body: Option<String>,
}

pub async fn process_job(
    priority: u8,
    platform: String,
    job: StreamId,
    url_gateway: String,
    callback_url: String,
    db_conn: PgPool,
    q_conn: MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    // 0. store values
    let mut n_id = -1;

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
                if key.eq("notification_id") {
                    n_id = s_value.parse::<i64>()?;
                } else if key.eq("event_id") {
                    notify_req.event_id = s_value;
                } else if key.eq("device_token") {
                    notify_req.device_token = s_value;
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

    println!("{:?}", response);
    // 4. From the gateway response, insert values in notification_deliverables table
    // println!();
    Ok(())
}
