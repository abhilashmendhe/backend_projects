use actix_web::http::StatusCode;
use reqwest::Client;
use serde::Deserialize;

use crate::utils::errors::{AppError, TinyUrlError};

#[derive(Debug, Deserialize)]
pub struct QueryResp {
    pub key: String,
    pub flag: bool,
}

pub async fn bloom_filter_query(key: String) -> Result<QueryResp, TinyUrlError> {
    let client = Client::new();
    let url = format!("http://localhost:3060/bf/v1?key={}", key);
    let _res = client.get(url).send().await.map_err(|err| {
        tracing::error!(
            "Error making request call to bloom filter server: {:?}",
            err
        );
        TinyUrlError::AppError(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ))
    })?;
    let query_resp = if let Ok(query_resp) = _res.text().await {
        let qres = serde_json::from_str::<QueryResp>(&query_resp).map_err(|err| {
            tracing::error!("Failed to query from bloom filter: {:?}", err);
            TinyUrlError::AppError(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            ))
        })?;
        qres
    } else {
        QueryResp { key, flag: true }
    };
    // println!("{:?}",_res.text().await);
    Ok(query_resp)
}
