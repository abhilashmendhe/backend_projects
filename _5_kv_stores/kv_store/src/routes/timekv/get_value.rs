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
    
    let value = {
        let tkv_lock_gaurd = tkv.lock().await;
        tkv_lock_gaurd.get(reqkv.key.clone(), reqkv.timestamp)
    };

    if value.len() <= 0 {
        info!(" ->> GET   /api/v1 {:>4}key:`{}`{:>2}404 NOT FOUND","",&reqkv.key,"");
        (StatusCode::NOT_FOUND, Json(GetResponse{value}))
    } else {
        info!(" ->> GET   /api/v1 {:>4}key:`{}`{:>2}200 OK","",&reqkv.key,"");
        (StatusCode::OK, Json(GetResponse{value}))
    }
}