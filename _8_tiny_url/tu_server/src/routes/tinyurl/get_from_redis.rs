use actix_web::http::StatusCode;
use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::{
    models::tinyurl_model::TinyUrlModel,
    utils::errors::{AppError, TinyUrlError},
};

pub async fn get_from_redis(
    mut conn: MultiplexedConnection,
    short_url_code: String,
) -> Result<TinyUrlModel, TinyUrlError> {
    match conn.get::<&str, String>(&short_url_code).await {
        Ok(tinyurl_payload) => {
            let tinyurl =
                serde_json::from_str::<TinyUrlModel>(&tinyurl_payload).map_err(|err| {
                    tracing::error!(
                        "Error converting from str to tinyurl model in redis get: {:?}",
                        err
                    );
                    TinyUrlError::AppError(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal server error",
                    ))
                })?;
            Ok(tinyurl)
        }
        Err(err) => {
            tracing::error!("Error tinyurl payload not found in redis cache: {:?}", err);
            return Err(TinyUrlError::AppError(AppError::new(
                StatusCode::NOT_FOUND,
                "Tiny url payload not found in redis.",
            )));
        }
    }
}
