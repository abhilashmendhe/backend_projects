use actix_web::{http::StatusCode, web};
use chrono::{DateTime, TimeDelta, Utc};

use crate::{
    routes::notify::{notify::SubNotificationResp, push_to_queue::push_to_redis_queue},
    utils::{
        app_state::AppState,
        errors::{AppError, NotificationServerErr},
    },
};

pub async fn retry_notification(
    not_deliv_id: i64,
    retry_count: u32,
    notification_id: i64,
    event_id: &str,
    device_token: &str,
    platform: &str,
    priority: i16,
    app_data: web::Data<AppState>,
) -> Result<SubNotificationResp, NotificationServerErr> {
    let mut sub_notification = SubNotificationResp {
        device_token: device_token.to_string(),
        platform: platform.to_string(),
        status: "FAILED".to_owned(),
    };
    let n_secs = 2_i64.pow(retry_count);
    let next_date_time = Utc::now() + TimeDelta::seconds(n_secs);
    // failed, check max_retry
    if priority == 0 {
        // high -- max_retry = 5
        if retry_count < 5 {
            // enqueue job
            sub_notification = update_next_retry_at(
                not_deliv_id,
                notification_id,
                event_id,
                device_token,
                platform,
                priority,
                next_date_time,
                app_data.clone(),
            )
            .await?;
        }
    } else {
        // low  -- max_retry = 3
        if retry_count < 3 {
            // enqueue job
            sub_notification = update_next_retry_at(
                not_deliv_id,
                notification_id,
                event_id,
                device_token,
                platform,
                priority,
                next_date_time,
                app_data.clone(),
            )
            .await?;
        }
    }

    Ok(sub_notification)
}

async fn update_next_retry_at(
    not_deliv_id: i64,
    notification_id: i64,
    event_id: &str,
    device_token: &str,
    platform: &str,
    priority: i16,
    next_date_time: DateTime<Utc>,
    app_data: web::Data<AppState>,
) -> Result<SubNotificationResp, NotificationServerErr> {
    let sub_notification = push_to_redis_queue(
        notification_id,
        event_id,
        &device_token,
        &platform,
        priority,
        app_data.clone(),
    )
    .await?;
    if sub_notification.status.eq("QUEUED") {
        // update next_retry_at
        sqlx::query!(
            r#"
                UPDATE notification_deliverables SET next_retry_at = $1 WHERE id = $2
            "#,
            next_date_time,
            not_deliv_id
        )
        .execute(app_data.db_pool())
        .await
        .map_err(|err| {
            // tracing::error!("-->\t {method} {path}  --  {:?}", err);
            tracing::error!("-->\t -- {:?}", err);
            return NotificationServerErr::AppError(AppError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to crate notification".to_string(),
            });
        })?;
    }
    Ok(sub_notification)
}
