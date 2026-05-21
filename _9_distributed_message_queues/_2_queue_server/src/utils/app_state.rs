use crate::models::payload_req::PayloadReq;

#[derive(Debug, Clone)]
pub struct AppState {
    tx: tokio::sync::mpsc::Sender<PayloadReq>,
}

impl AppState {
    pub fn new(tx: tokio::sync::mpsc::Sender<PayloadReq>) -> Self {
        Self { tx }
    }
    pub fn tx(&self) -> tokio::sync::mpsc::Sender<PayloadReq> {
        self.tx.clone()
    }
}
