use crate::{
    services::{BuffEvents, parse_string_array_to_f64},
    utils::error::ExchangeErr,
};
use reqwest::StatusCode;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Deserialize)]
pub struct HttpResponseSnapshot {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: usize,

    #[serde(deserialize_with = "parse_string_array_to_f64")]
    pub bids: Vec<[Decimal; 2]>,
    #[serde(deserialize_with = "parse_string_array_to_f64")]
    pub asks: Vec<[Decimal; 2]>,
}

pub async fn fetch_snapshot(
    rest_url: String,
    sender: Sender<BuffEvents>,
) -> Result<(), ExchangeErr> {
    let resp = reqwest::get(rest_url).await?;
    // .json::<HashMap<String, String>>()
    // .await?;
    if resp.status() != StatusCode::OK {
        return Err(ExchangeErr::HttpResponseErr(format!("{}", resp.status())));
    }
    let json = resp.json::<HttpResponseSnapshot>().await?;
    println!("{:#?}", json);
    let _ = sender.send(BuffEvents::Snapshot(json)).await;
    Ok(())
}
