use chrono::Utc;
use serde::Serialize;
// use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::payload_req::{Payload, PayloadReq};

#[derive(Debug, Serialize)]
pub enum Status {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED,
}

#[derive(Debug, Serialize)]
pub struct SendPayload {
    pub id: Uuid,
    pub payload: Payload,
    pub status: Status, // pending|processing|completed|failed
    pub priority: u32,
    pub retries: u32,
    pub max_retries: u32,
    pub created_at: i64,
    pub run_at: i64,
    pub last_error: Option<String>,
    pub worker_id: Option<u32>,
    pub processing_started_at: Option<i64>,
}

impl SendPayload {
    pub fn new(payload_req: PayloadReq, retries: u32, max_retries: u32) -> SendPayload {
        Self {
            id: Uuid::new_v4(),
            payload: payload_req.payload,
            status: Status::PENDING,
            priority: payload_req.priority,
            retries,
            max_retries,
            created_at: Utc::now().timestamp_millis(),
            run_at: payload_req.run_at,
            last_error: None,
            worker_id: None,
            processing_started_at: None,
        }
    }
}
