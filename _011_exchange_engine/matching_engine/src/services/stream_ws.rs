use crate::{
    services::{BuffEvents, parse_string_array_to_f64},
    utils::error::ExchangeErr,
};
use futures_util::StreamExt;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::time::Duration;
use tokio::{sync::mpsc::Sender, time::timeout};
use tokio_tungstenite::connect_async;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct WSResponse {
    pub e: String, // Event type, in this case, depthUpdate
    pub E: usize,  // Event time at which the request was made in milliseconds
    pub s: String, // Symbol requested, e.g. BNBUSDT
    pub U: usize,  // First update ID in the event
    pub u: usize,  // Final update ID in the event

    #[serde(deserialize_with = "parse_string_array_to_f64")]
    pub b: Vec<[Decimal; 2]>, // Array of bid updates, where each entry contains [price, quantity]

    #[serde(deserialize_with = "parse_string_array_to_f64")]
    pub a: Vec<[Decimal; 2]>, // Array of ask updates, where each entry contains [price, quantity]
}

pub async fn start_streaming(
    url: String,
    connection_timeout: Duration,
    sender: Sender<BuffEvents>,
) -> Result<(), ExchangeErr> {
    match timeout(connection_timeout, connect_async(url)).await {
        Ok(Ok((ws_stream, _response))) => {
            // for (ref header, _value) in response.headers() {
            //     println!("* {}", header);
            // }

            let (_, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        let ws_response = serde_json::from_str::<WSResponse>(&msg.to_string())?;
                        // println!("{:?}", ws_response);
                        let _ = sender.send(BuffEvents::WSStream(ws_response)).await;
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed due to WebSocket/TCP error: {}", e);
        }
        Err(_) => {
            eprintln!(
                "Connection attempt timed out after {:?}",
                connection_timeout
            );
            // Err(tokio_tungstenite::tungstenite::Error::Io(
            //     std::io::Error::new(std::io::ErrorKind::TimedOut, "Connect timeout")
            // ))
        }
    }
    Ok(())
}
