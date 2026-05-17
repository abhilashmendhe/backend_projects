use axum::{Json, extract::State, http::StatusCode};
use bloomfilter::bf::BloomFilter;
use serde::Deserialize;

use crate::utils::error::{AppError, BloomFilterErr};

#[derive(Debug, Deserialize)]
pub struct InsertReq {
    pub key: String,
}

pub async fn insert_key(
    State(bloom_filter): State<BloomFilter>,
    Json(insert_req): Json<InsertReq>,
) -> Result<(StatusCode, String), BloomFilterErr> {
    let key = insert_req.key;
    bloom_filter.insert(&key).await.map_err(|err| {
        tracing::error!("Error inserting key in BF: {:?}", err);
        BloomFilterErr::BFCustomErr(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error. (err inserting key in BF)",
        ))
    })?;
    Ok((StatusCode::CREATED, format!("`{}` inserted in BF!", key)))
}
