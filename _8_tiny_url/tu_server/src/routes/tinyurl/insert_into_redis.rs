use actix_web::http::StatusCode;
use redis::{AsyncTypedCommands, aio::MultiplexedConnection};

use crate::{
    models::tinyurl_model::TinyUrlModel,
    utils::errors::{AppError, TinyUrlError},
};

pub async fn insert_into_redis(
    mut conn: MultiplexedConnection,
    short_url_code: String,
    tiny_url_model: &TinyUrlModel,
) -> Result<(), TinyUrlError> {
    let tiny_url_payload = serde_json::to_string(tiny_url_model).map_err(|err| {
        tracing::error!(
            "Error converting from tiny_url_model to string in insert to redis: {:?}",
            err
        );
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ))
    })?;
    conn.set::<String, String>(short_url_code, tiny_url_payload)
        .await
        .map_err(|err| {
            tracing::error!("Error inserting into redis cache: {:?}", err);
            TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            ))
        })?;
    Ok(())
}
