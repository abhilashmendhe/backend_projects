use actix_web::{HttpResponse, http::StatusCode, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;

use crate::{
    models::user_model::UserModel,
    routes::tinyurl::{
        TURL,
        bloom_filter_insert::bloom_filter_insert,
        helpers::{encode_short_url::encode_short_url, serialize_date_time::FDateTime},
    },
    utils::{
        app_state::AppState,
        errors::{AppError, TinyUrlError},
    },
};
use validator::Validate;

/**
* 1. Insert into DB
       ↓
* 2. Commit succeeds
       ↓
* 3. Add to Bloom filter
*/

#[derive(Debug, Deserialize, Validate)]
pub struct TinyUrlRequest {
    #[validate(url)]
    long_url: String,
    expired_at: Option<FDateTime>,
}

#[derive(Debug, Serialize)]
pub struct TinyUrlResponse {
    short_url: String,
}

pub async fn create_short_url(
    user: UserModel,
    tu_req: web::Json<TinyUrlRequest>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, TinyUrlError> {
    // 1. check if url was passed empty or not, and also check if it was a bad url
    let _ = &tu_req.validate().map_err(|_err| {
        TinyUrlError::AppError(AppError::new(
            StatusCode::BAD_REQUEST,
            "Either URL was passed empty or a bad url.",
        ))
    })?;

    // 2. Get short url code
    let short_url_code = encode_short_url();
    let short_url = format!("{}/{}", TURL, &short_url_code);

    let mut qb = QueryBuilder::new("INSERT INTO tinyurl(user_id,short_url_code,long_url");
    if let Some(date) = &tu_req.expired_at {
        let d = match date {
            FDateTime::Date(date) => {
                DateTime::<Utc>::from_naive_utc_and_offset(date.and_hms_opt(0, 0, 0).unwrap(), Utc)
            }
            FDateTime::DateTime(date_time) => date_time.clone(),
        };
        qb.push(",expired_at) VALUES(");
        qb.push_bind(user.id as i32);
        qb.push(", ");
        qb.push_bind(&short_url_code);
        qb.push(", ");
        qb.push_bind(&tu_req.long_url);
        qb.push(", ");
        qb.push_bind(d);
        qb.push(")");
    } else {
        qb.push(") VALUES(");
        qb.push_bind(user.id as i32);
        qb.push(", ");
        qb.push_bind(&short_url_code);
        qb.push(", ");
        qb.push_bind(&tu_req.long_url);
        qb.push(")");
    }

    let pool = app_data.pool();
    let mut tx = pool.begin().await.map_err(|err| {
        println!("transaction err: {err}");
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error.",
        ))
    })?;
    qb.build().execute(&mut *tx).await.map_err(|err| {
        tracing::error!("{err}");
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error at (creating tiny url)",
        ))
    })?;
    tx.commit().await.map_err(|err| {
        println!("transaction err: {err}");
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error.",
        ))
    })?;

    //  Now insert it into bloom filter
    bloom_filter_insert(short_url_code).await?;

    Ok(HttpResponse::Ok().json(TinyUrlResponse { short_url }))
}
