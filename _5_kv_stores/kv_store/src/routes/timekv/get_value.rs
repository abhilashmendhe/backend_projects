use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use tokio::sync::Mutex;
use tracing::info;

use crate::{data::kvt_struct::TimeBasedKV, routes::timekv::GetRequestKV};

#[derive(Debug, Serialize)]
pub struct GetResponse {
    pub value: String
}

pub async fn get_value(
    State(tkv): State<Arc<Mutex<TimeBasedKV>>>,
    Json(reqkv): Json<GetRequestKV>
) -> (StatusCode, Json<GetResponse>) {
    
    info!(" ->> GET   /api/v1 {:>4}key:`{}`","",&reqkv.key);
    let value = {
        let tkv_lock_gaurd = tkv.lock().await;
        tkv_lock_gaurd.get(reqkv.key, reqkv.timestamp)
    };

    if value.len() <= 0 {
        (StatusCode::NOT_FOUND, Json(GetResponse{value}))
    } else {
        (StatusCode::OK, Json(GetResponse{value}))
    }
}