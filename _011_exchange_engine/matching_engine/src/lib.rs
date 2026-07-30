use std::time::Duration;

use crate::{
    services::{BuffEvents, fetch_rest::fetch_snapshot, stream_ws::start_streaming},
    utils::error::ExchangeErr,
};

pub mod orderbook;
pub mod services;
pub mod utils;

pub async fn build_orderbook(
    rest_url: String,
    ws_url: String,
    connection_timeout: u64,
) -> Result<(), ExchangeErr> {
    println!("Before spawn");

    // 1. create mpsc channel
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<BuffEvents>(10);

    // 2. Applying incremental updates from the WebSocket stream
    let sender2 = sender.clone();
    let _ = tokio::spawn(async move {
        println!("kya backchodi hai");
        let _ = start_streaming(
            ws_url,
            Duration::from_secs(connection_timeout),
            sender2.clone(),
        )
        .await;
    });

    println!("After spawn");
    println!("Before snapshot");
    // 3. Fetch initial snapshot via REST
    fetch_snapshot(rest_url, sender.clone()).await?;

    println!("After snapshot");

    println!("Reading from receiver");
    while let Some(buff_events) = receiver.recv().await {
        println!("{:?}", buff_events);
    }
    Ok(())
}
