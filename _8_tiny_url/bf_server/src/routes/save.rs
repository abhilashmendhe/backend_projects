use axum::{Json, extract::State, http::StatusCode};
use bloomfilter::bf::BloomFilter;
use serde::Deserialize;

use crate::utils::error::{AppError, BloomFilterErr};


#[derive(Debug, Deserialize)]
pub struct SaveReq {
    pub file_path: String
}
pub async fn save(
    State(bloom_filter): State<BloomFilter>,
    Json(save_req): Json<SaveReq>,
) -> Result<StatusCode, BloomFilterErr> {

    bloom_filter.save(&save_req.file_path).await.map_err(|err| {
        tracing::error!("Error saving BF: {:?}", err);
        BloomFilterErr::BFCustomErr(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error. (err saving BF)",
        ))
    })?;
    Ok(StatusCode::CREATED)
}