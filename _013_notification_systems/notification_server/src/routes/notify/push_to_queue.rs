use crate::utils::{
    app_state::AppState,
    errors::{AppError, NotificationServerErr},
};
use actix_web::{http::StatusCode, web};
use redis::{AsyncTypedCommands, aio::MultiplexedConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub notification_id: i64,
    pub event_id: String,
    pub device_token: String,
}

pub async fn push_to_redis_queue(
    notification_id: i64,
    event_id: &str,
    device_token: &str,
    platform: &str,
    priority: i16,
    app_data: web::Data<AppState>,
) -> Result<(), NotificationServerErr> {
    let mut conn = if platform.eq("ios") {
        app_data.redis_ios_q()
    } else if platform.eq("android") {
        app_data.redis_android_q()
    } else {
        return Err(NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Nowhere to send in the message queue.".to_string(),
        }));
    };

    let message = serde_json::to_string(&Message {
        notification_id,
        event_id: event_id.to_string(),
        device_token: device_token.to_string(),
    })
    .map_err(|_err| {
        // tracing::error!("-->\t {method} {path}  --  {:?}", err);
        return NotificationServerErr::AppError(AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to convert from Message to JSON string".to_string(),
        });
    })?;

    let re = conn
        .lpush::<String, String>(priority.to_string(), message)
        .await;
    Ok(())
}
