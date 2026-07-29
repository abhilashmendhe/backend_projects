use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Deserialize;

use crate::utils::error::ExchangeErr;

#[derive(Debug, Deserialize)]
pub struct HttpResponseSnapshot {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: usize,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>
}

pub async fn fetch_snapshot(rest_url: String) -> Result<(), ExchangeErr> {

    let resp = reqwest::get(rest_url).await?;
    // .json::<HashMap<String, String>>()
    // .await?;

    if resp.status() != StatusCode::OK {
        return Err(ExchangeErr::HttpResponseErr(format!("{}", resp.status())));
    }
    // println!("{:#?}", resp);
    let json = resp.json::<HttpResponseSnapshot>().await?;
    println!("{:?}",json);
    Ok(())
}
