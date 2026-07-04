use actix_web::{HttpRequest, HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    models::device::Device,
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
pub struct NotificationResponse {
    pub notification_id: i64,
    pub event_id: String,
    pub device_tokens: Vec<String>,
    pub platforms: Vec<String>,
    pub status: String,
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
            message: format!("Devices not registered with user id:{}", user_id),
        }));
    }

    // 3. Create a notification
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
                        message: format!("Notification already exists with event_id:{}!", event_id),
                    });
                }
            }
        }
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to crate notification".to_string(),
        });
    })?;

    // 4. Send info to respective message queues (`ios` and `android`)
    let mut device_tokens = vec![];
    let mut platforms = vec![];
    for device in devices {
        if let Some(act) = device.is_active {
            if act {
                // push to redis queue.. 
                
                device_tokens.push(device.device_token);
                platforms.push(device.platform);
            }
        }
    }

    Ok(HttpResponse::Ok().json(NotificationResponse {
        notification_id,
        event_id: event_id.to_string(),
        device_tokens,
        platforms,
        status: "QUEUED".to_string(),
    }))
}
