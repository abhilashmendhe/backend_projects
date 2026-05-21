use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "job_type", content = "payload")]
pub enum Payload {
    #[serde(rename = "send_email")]
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },

    #[serde(rename = "process_image")]
    ProcessImage {
        image_url: String,
        resize: (u32, u32),
    },

    #[serde(rename = "generate_report")]
    GenerateReport { desc: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayloadReq {
    #[serde(flatten)]
    pub payload: Payload,
    pub priority: u32,
    pub run_at: i64,
}

impl PayloadReq {
    pub fn job_type(&self) -> String {
        match self.payload {
            Payload::SendEmail { .. } => "send_email".to_string(),
            Payload::ProcessImage { .. } => "process_image".to_string(),
            Payload::GenerateReport { .. } => "generate_report".to_string(),
        }
    }
}
