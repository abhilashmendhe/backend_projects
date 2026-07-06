use actix_web::{HttpRequest, HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::{
    app_state::AppState,
    errors::{AppError, NotificationServerErr},
};

/**
 *
    $ curl -X POST localhost:60001/notify \
    > -H "Content-Type: application/json" \
    > -d '{"event_id": "evt_abc123", "recipient_user_id": "user_0600", "title": "Fall detected, Room 14", "body": "Mrs. Hansen, please check immediately", "priority": "high", "occurred_at": "2026-05-23T18:53:49Z"}'
*/

// --------------------- Users and Deivces combined data model --------------------
#[derive(Debug, Deserialize)]
pub struct UserDevice {
    pub u_id: i64,
    pub u_email: String,
    pub d_id: i64,
    pub d_device_token: String,
    pub d_platform: String,
    pub d_last_seen_at: Option<DateTime<Utc>>,
    pub d_is_active: Option<bool>,
}

// --------------------- Request JSON --------------------
#[derive(Debug, Deserialize)]
pub struct NotificationRequest {
    pub event_id: String,
    pub recipient_user_id: String,
    pub title: String,
    pub body: String,
    pub priority: String,
    pub occurred_at: DateTime<Utc>,
}

// --------------------- Response JSON --------------------
#[derive(Debug, Serialize)]
pub struct Platform {
    pub platform: String,
    pub is_active: Option<bool>,
    pub last_seen_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SubNotificationResp {
    pub user_id: i64,
    pub notification_id: i64,
    pub outbox_id: i64,
    pub event_id: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub message: String,
    pub sub_notification_resp: SubNotificationResp,
    pub platforms: Vec<Platform>,
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
    let priority_str = &req_event.priority.to_lowercase();
    let created_at = &req_event.occurred_at;

    if event_id.eq("") || r_user_id.eq("") || title.eq("") || priority_str.eq("") {
        return Err(NotificationServerErr::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Incomplete event details!!!",
        )));
    }
    let priority = if priority_str.eq("high") { 0 } else { 1 };

    // 2. Check if user and devices exists with inner join query
    /*
        SELECT
            u.id AS u_id,
            u.email AS u_email,
            d.id AS d_id,
            d.device_token AS d_device_token,
            d.platform AS d_platform,
            d.last_seen_at AS d_last_seen_at,
            d.is_active AS d_is_active
        FROM users AS u
        INNER JOIN devices AS d
        ON u.id=d.user_id WHERE u.username='user_0041';
    */
    let users_devices = sqlx::query_as!(
        UserDevice,
        r#"
        SELECT 
            u.id AS u_id,
            u.email AS u_email,
            d.id AS d_id,
            d.device_token AS d_device_token,
            d.platform AS d_platform,
            d.last_seen_at AS d_last_seen_at,
            d.is_active AS d_is_active
        FROM users AS u 
        INNER JOIN devices AS d 
        ON u.id=d.user_id WHERE u.username=$1
        "#,
        r_user_id
    )
    .fetch_all(app_data.db_pool())
    .await
    .map_err(|err| {
        tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to get user".to_string(),
        });
    })?;
    if users_devices.len() == 0 {
        return Err(NotificationServerErr::AppError(AppError {
            code: StatusCode::NOT_FOUND,
            message: format!("User with id: {r_user_id} is not registered"),
        }));
    }
    let mut platforms = vec![];
    for ud in &users_devices {
        platforms.push(Platform {
            platform: ud.d_platform.clone(),
            is_active: ud.d_is_active,
            last_seen_at: ud.d_last_seen_at,
        });
    }
    let user_id = users_devices[0].u_id;
    // 3. Insert data into notifications and notification_outbox table with a transaction
    let mut tx = app_data.db_pool().begin().await.map_err(|err| {
        tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::SERVICE_UNAVAILABLE,
            message: "Failed to begin with transactions.".to_string(),
        });
    })?;

    // 3.1 Insert into notification table
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
    .fetch_one(&mut *tx)
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

    // 3.2 Insert into outbox table
    let outbox_id = sqlx::query_scalar!(
        r#"
        INSERT INTO notification_outbox(notification_id, user_id) 
        VALUES ($1, $2)
        RETURNING id 
        "#,
        notification_id,
        user_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| {
        tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to crate notification outbox".to_string(),
        });
    })?;
    tx.commit().await.map_err(|err| {
        tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::SERVICE_UNAVAILABLE,
            message: "Failed to commit transactions. Rolling back.".to_string(),
        });
    })?;
    Ok(HttpResponse::Ok().json(NotificationResponse {
        message: "Notification create.".to_owned(),
        sub_notification_resp: SubNotificationResp {
            user_id,
            notification_id,
            outbox_id,
            event_id: event_id.to_string(),
        },
        platforms,
    }))
}
