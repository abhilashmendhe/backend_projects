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
    HttpResponse, ResponseError,
    http::StatusCode,
    web,
};

use crate::{
    models::{tinyurl_model::TinyUrlModel, user_model::UserModel},
    routes::tinyurl::{
        bloom_filter_query::bloom_filter_query, get_from_redis::get_from_redis,
        helpers::redirect_fn::redirect_fn, insert_into_redis::insert_into_redis,
    },
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
    match get_from_redis(app_data.redis_conn().clone(), key.to_string()).await {
        Ok(tiny_url_m) => return Ok(redirect_fn(tiny_url_m).await?),
        Err(err) => {
            if err.status_code() == StatusCode::INTERNAL_SERVER_ERROR {
                return Err(err);
            }
        }
    }

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
    // 4. Now insert the found data from pg to redis
    insert_into_redis(app_data.redis_conn().clone(), key, &tiny_url_m).await?;
    Ok(redirect_fn(tiny_url_m).await?)
}
