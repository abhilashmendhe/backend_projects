use actix_web::http::StatusCode;
use reqwest::Client;
use serde::Serialize;

use crate::utils::errors::{AppError, TinyUrlError};

#[derive(Debug, Serialize)]
struct InsertReq {
    key: String,
}

pub async fn bloom_filter_insert(key: String) -> Result<(), TinyUrlError> {
    let client = Client::new();
    let _res = client
        .post("http://localhost:3060/bf/v1")
        .json(&InsertReq { key })
        .send()
        .await
        .map_err(|err| {
            tracing::error!(
                "Error making request call to bloom filter server: {:?}",
                err
            );
            TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            ))
        })?;
    // println!("{:?}",res);
    Ok(())
}
