use actix_web::{HttpRequest, HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    models::{
        device::Device, notification::Notification,
        notification_deliverables::NotificationDeliverables,
    },
    routes::notify::{notification_retry::retry_notification, push_to_queue::push_to_redis_queue},
    utils::{
        app_state::AppState,
        errors::{AppError, NotificationServerErr},
    },
};

/**
 *
    $ curl -X POST localhost:60001/notify \
    > -H "Content-Type: application/json" \
    > -d '{"event_id": "evt_abc123", "recipient_user_id": "user_42", "title": "Fall detected, Room 14", "body": "Mrs. Hansen, please check immediately", "priority": "high", "occurred_at": "2026-05-23T18:53:49Z"}'
*/

#[derive(Debug)]
pub enum ForNotification {
    Notification(Notification),
    NotFoundNotification,
}

#[derive(Debug, Deserialize)]
pub struct NotificationRequest {
    pub event_id: String,
    pub recipient_user_id: String,
    pub title: String,
    pub body: String,
    pub priority: String,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SubNotificationResp {
    pub device_token: String,
    pub platform: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub user_id: i64,
    pub notification_id: i64,
    pub event_id: String,
    pub sub_notification_resp: Vec<SubNotificationResp>,
}

pub async fn notify_event(
    req: HttpRequest,
    req_event: web::Json<NotificationRequest>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, NotificationServerErr> {
    let path = req.path();
    let method = req.method();
    tracing::info!("-->\t {method} {path}");
    // 1. Check if received event is empty or not
    let event_id = &req_event.event_id;
    let r_user_id = &req_event.recipient_user_id;
    let title = &req_event.title;
    let body = &req_event.body;
    let priority_str = &req_event.priority;
    let created_at = &req_event.occurred_at;

    if event_id.eq("") || r_user_id.eq("") || title.eq("") || priority_str.eq("") {
        return Err(NotificationServerErr::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Incomplete event details!!!",
        )));
    }
    let priority = if priority_str.to_lowercase().eq("high") {
        0
    } else {
        1
    };
    // 2. Check if user exists and device exists (think if fetching both with inner joins)
    // 2.1 get user id if exists
    let user_id = sqlx::query_scalar!(r#"SELECT id FROM users WHERE username=$1"#, r_user_id)
        .fetch_one(app_data.db_pool())
        .await
        .map_err(|err| {
            tracing::error!("-->\t {method} {path}  --  {:?}", err);
            return NotificationServerErr::AppError(AppError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to get user".to_string(),
            });
        })?;

    // 2.2 fetch all devices associated with user_id
    let devices = sqlx::query_as!(
        Device,
        r#"
            SELECT * FROM devices WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(app_data.db_pool())
    .await
    .map_err(|err| {
        tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to get device information".to_string(),
        });
    })?;

    if devices.len() == 0 {
        return Err(NotificationServerErr::AppError(AppError {
            code: StatusCode::GONE,
            message: format!("Devices not registered/deleted with user id:{}", user_id),
        }));
    }

    // 3. First check if event_id(global) exists in the table
    let rnot = match sqlx::query_as!(
        Notification,
        r#"
            SELECT * FROM notifications WHERE event_id = $1
        "#,
        event_id
    )
    .fetch_one(app_data.db_pool())
    .await
    {
        Ok(notification) => ForNotification::Notification(notification),
        Err(_) => ForNotification::NotFoundNotification,
    };

    // 4. Create a notification if not found else check notification deliverables
    let (notification_id, _status) = match rnot {
        ForNotification::Notification(notification) => {
            // println!("notification exists.... now checking deliverables table");
            // 4.1 check deliverable table
            // 4.1.1 if found, do checks
            let noti_deliv = sqlx::query_as!(
                NotificationDeliverables,
                r#"
                    SELECT * FROM notification_deliverables WHERE notification_id = $1
                "#,
                notification.id
            )
            .fetch_all(app_data.db_pool())
            .await
            .map_err(|err| {
                tracing::error!("-->\t {method} {path}  --  {:?}", err);
                return NotificationServerErr::AppError(AppError {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch from notification deliverables table!!".to_string(),
                });
            })?;

            if noti_deliv.len() > 0 {
                let mut sub_notifications = vec![];
                for nd in noti_deliv {
                    let device = devices.iter().find(|d| d.id == nd.device_id);
                    let (device_token, platform) = if let Some(d) = device {
                        (d.device_token.to_string(), d.platform.to_string())
                    } else {
                        ("".to_string(), "".to_string())
                    };
                    // if status is not pending, return response to the client
                    if nd.status == 0 {
                        // sent
                        sub_notifications.push(SubNotificationResp {
                            device_token: device_token,
                            platform: platform,
                            status: "SENT".to_owned(),
                        });
                    } else if nd.status == 1 {
                        let sub_notification = retry_notification(
                            nd.id,
                            nd.retry_count as u32,
                            nd.notification_id,
                            event_id,
                            &device_token,
                            &platform,
                            priority,
                            app_data.clone(),
                        )
                        .await?;
                        sub_notifications.push(sub_notification);
                    } else {
                        // pending
                        sub_notifications.push(SubNotificationResp {
                            device_token: device_token,
                            platform: platform,
                            status: "PENDING".to_owned(),
                        });
                    }
                    return Ok(HttpResponse::Ok().json(NotificationResponse {
                        user_id,
                        notification_id: nd.notification_id,
                        event_id: event_id.to_string(),
                        sub_notification_resp: sub_notifications,
                    }));
                }
            }
            // return Err(NotificationServerErr::AppError(AppError {
            //     code: StatusCode::INTERNAL_SERVER_ERROR,
            //     message: "Failed to crate notification".to_string(),
            // }));

            // 4.1.2 if not found enqueu in the message queue
            (notification.id, "QUEUED".to_string())
        }
        ForNotification::NotFoundNotification => {
            // 4.1 create notification and enque in the message queue
            let notification_id = sqlx::query_scalar!(
                r#"
        INSERT INTO notifications(user_id, event_id, title, body, payload, priority, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
                user_id,
                event_id,
                title,
                body,
                Some(json!({})),
                priority,
                created_at
            )
            .fetch_one(app_data.db_pool())
            .await
            .map_err(|err| {
                tracing::error!("-->\t {method} {path}  --  {:?}", err);
                if let Some(pg_err) = err.as_database_error() {
                    if let Some(e_code) = pg_err.code() {
                        if e_code.to_string().eq("23505") {
                            return NotificationServerErr::AppError(AppError {
                                code: StatusCode::BAD_REQUEST,
                                message: format!(
                                    "Notification already exists with event_id:{}!",
                                    event_id
                                ),
                            });
                        }
                    }
                }
                return NotificationServerErr::AppError(AppError {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to crate notification".to_string(),
                });
            })?;
            (notification_id, "QUEUED".to_string())
        }
    };

    // 5. Send info to respective message queues (`ios` and `android`)
    let mut sub_notifications = vec![];
    for device in devices {
        if let Some(act) = device.is_active {
            if act {
                // push to redis queue..
                let sub_notification = push_to_redis_queue(
                    notification_id,
                    event_id,
                    &device.device_token,
                    &device.platform,
                    priority,
                    app_data.clone(),
                )
                .await?;
                sub_notifications.push(sub_notification);
            }
        }
    }

    Ok(HttpResponse::Ok().json(NotificationResponse {
        user_id,
        notification_id,
        event_id: event_id.to_string(),
        sub_notification_resp: sub_notifications,
    }))
}
