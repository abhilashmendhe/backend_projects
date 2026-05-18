use actix_web::{
    HttpResponse,
    http::{StatusCode, header},
};
use chrono::Utc;

use crate::{
    models::tinyurl_model::TinyUrlModel,
    utils::errors::{AppError, TinyUrlError},
};

pub async fn redirect_fn(tiny_url_m: TinyUrlModel) -> Result<HttpResponse, TinyUrlError> {
    let r = if let Some(exp_date) = tiny_url_m.expired_at {
        // println!("expirey exists: {:?}", exp_date);
        let exp_date_unix = exp_date.timestamp();
        let curr_unix = Utc::now().timestamp();
        if curr_unix > exp_date_unix {
            return Err(TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "tiny url expired",
            )));
        }
        HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, tiny_url_m.long_url))
            .await
            .map_err(|err| {
                tracing::error!("Failed to redicret: {:?}", err);
                TinyUrlError::AppError(AppError::new(StatusCode::NOT_FOUND, "Not found"))
            })?
    } else {
        HttpResponse::PermanentRedirect()
            .append_header((header::LOCATION, tiny_url_m.long_url))
            .await
            .map_err(|err| {
                tracing::error!("Failed to redicret: {:?}", err);
                TinyUrlError::AppError(AppError::new(StatusCode::NOT_FOUND, "Not found"))
            })?
    };
    Ok(r)
}
