use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use tokio::sync::Mutex;
use tracing::info;

use crate::{data::kvt_struct::TimeBasedKV, routes::timekv::PutRequestKV};

pub async fn put_value(
    State(tkv): State<Arc<Mutex<TimeBasedKV>>>,
    Json(preqkv): Json<PutRequestKV>
) -> StatusCode {
    let s = format!("key:`{}`, value:`{}`, timestamp:`{}`",&preqkv.key, &preqkv.value, &preqkv.timestamp);
    info!(" ->> PUT   / {:>8}{{{}}}","",s);
    {
        let mut tkv_gaurd = tkv.lock().await;
        tkv_gaurd.set(preqkv.key, preqkv.value, preqkv.timestamp);
    }
    StatusCode::CREATED
}