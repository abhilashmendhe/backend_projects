/*
    GET /abc123
        ↓
    Bloom filter check
        ↓ maybe exists
    Redis lookup
        ↓ miss
    Postgres lookup
*/

use actix_web::{
    HttpResponse,
    http::{StatusCode, header},
    web,
};
use chrono::Utc;

use crate::{
    models::{tinyurl_model::TinyUrlModel, user_model::UserModel},
    routes::tinyurl::bloom_filter_query::bloom_filter_query,
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
    },
};

pub async fn get_redirect_short_url(
    user: UserModel,
    key_path: web::Path<String>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    // 0. extract short url code from path
    let key = key_path.into_inner();

    // 1. check in bloom filter server
    let bf_query_resp = bloom_filter_query(key.clone()).await?;
    if !bf_query_resp.flag {
        return Err(TinyUrlError::AppError(AppError::new(
            StatusCode::NOT_FOUND,
            "URL does not exists",
        )));
    }
    // 2. if true, then check with redis

    // 3. not found in redis cache, then sql to postgresql
    let tiny_url_m: TinyUrlModel = sqlx::query_as!(
        TinyUrlModel,
        r#"
            SELECT * FROM tinyurl WHERE user_id=$1 and short_url_code=$2 
        "#,
        user.id as i32,
        key
    )
    .fetch_one(app_data.pool())
    .await
    .map_err(|err| {
        tracing::error!("Failed to get short code url from dB: {:?}", err);
        TinyUrlError::AppError(AppError::new(StatusCode::NOT_FOUND, "URL does not exists"))
    })?;
    // println!("tiny url model: {:?}", tiny_url_m);

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
