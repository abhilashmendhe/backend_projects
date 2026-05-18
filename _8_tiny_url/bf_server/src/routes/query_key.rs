use axum::{Json, extract::{Query, State}, http::StatusCode};
use bloomfilter::bf::BloomFilter;
use serde::{Deserialize, Serialize};

use crate::utils::error::{AppError, BloomFilterErr};

#[derive(Debug, Deserialize)]
pub struct QueryReq {
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct QueryResp {
    pub key: String,
    pub flag: bool,
}

pub async fn query_key(
    State(bloom_filter): State<BloomFilter>,
    Query(query_req): Query<QueryReq>,
) -> Result<(StatusCode, Json<QueryResp>), BloomFilterErr> {
    let res = bloom_filter.query(&query_req.key).await.map_err(|err| {
        tracing::error!("Error querying key in BF: {:?}", err);
        BloomFilterErr::BFCustomErr(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error. (err querying key in BF)",
        ))
    })?;
    tracing::info!(" ->> GET /bf/v1 `{}` queried in BF!", &query_req.key);
    Ok((
        StatusCode::OK,
        Json(QueryResp {
            key: res.0,
            flag: res.1,
        }),
    ))
}
