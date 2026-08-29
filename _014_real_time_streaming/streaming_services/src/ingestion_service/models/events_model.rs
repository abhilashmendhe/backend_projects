use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Heartbeat,
    Presence,
    Motion,
    SleepState,
    FallWarn,
    NetStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepState {
    Asleep,
    Awake,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub device_id: String,
    pub room_id: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub ts: DateTime<Utc>,
    pub seq: usize,

    pub in_room: Option<bool>,
    pub magnitude: Option<f32>,
    pub state: Option<SleepState>,
    pub confidence: Option<f32>,
    pub rssi: Option<i32>,
}
