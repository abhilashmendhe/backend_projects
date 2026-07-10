use crate::utils::error::NotificationWorkerErr;
use reqwest::Response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotifyRequest {
    pub device_token: String,
    pub platform: String, // ios | android | sms | web
    pub event_id: String,
    pub title: String,
    pub body: String,
    pub priority: String,     // high | normal
    pub callback_url: String, // optional, gateway POSTs delivery confirmation here
}

impl NotifyRequest {
    pub fn new() -> Self {
        Self {
            device_token: "".to_string(),
            platform: "".to_string(),
            event_id: "".to_string(),
            title: "".to_string(),
            body: "".to_string(),
            priority: "".to_string(),
            callback_url: "".to_string(),
        }
    }
}

pub async fn make_gateway_request(
    url_gateway: String,
    notify_req: NotifyRequest,
) -> Result<Response, NotificationWorkerErr> {
    let url = format!("http://{}", url_gateway);
    let client = reqwest::Client::new();
    Ok(client.post(url).json(&notify_req).send().await?)
}
