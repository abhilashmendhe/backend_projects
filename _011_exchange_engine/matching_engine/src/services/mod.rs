use rust_decimal::Decimal;
use serde::de::{Deserialize, Deserializer};

use crate::services::{fetch_rest::HttpResponseSnapshot, stream_ws::WSResponse};

pub mod fetch_rest;
pub mod stream_ws;

#[derive(Debug)]
pub enum BuffEvents {
    Snapshot(HttpResponseSnapshot),
    WSStream(WSResponse),
}

pub fn parse_string_array_to_f64<'de, D>(deserializer: D) -> Result<Vec<[Decimal; 2]>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_vec: Vec<[String; 2]> = Vec::deserialize(deserializer)?;
    raw_vec
        .iter()
        .map(|[price_st, qt_st]| {
            let price = price_st
                .parse::<Decimal>()
                .map_err(serde::de::Error::custom)?;
            let qty = qt_st.parse::<Decimal>().map_err(serde::de::Error::custom)?;
            Ok([price, qty])
        })
        .collect()
}
