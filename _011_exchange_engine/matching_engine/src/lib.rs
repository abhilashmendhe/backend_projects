use crate::{
    services::{
        BuffEvents, fetch_rest::fetch_snapshot, process_buff_events::process_events,
        stream_ws::start_streaming,
    },
    utils::error::ExchangeErr,
};

pub mod orderbook;
pub mod services;
pub mod utils;

pub async fn build_orderbook(
    ticker: String,
    rest_url: String,
    ws_url: String,
    connection_timeout: u64,
) -> Result<(), ExchangeErr> {
    // println!("Before spawn");

    // 1. create mpsc channel
    let (sender, receiver) = tokio::sync::mpsc::channel::<BuffEvents>(10);

    // 2. Applying incremental updates from the WebSocket stream
    let sender2 = sender.clone();
    let _ = tokio::spawn(async move {
        let _ = start_streaming(
            ws_url,
            std::time::Duration::from_secs(connection_timeout),
            sender2.clone(),
        )
        .await;
    });

    // println!("After spawn");
    // println!("Before snapshot");
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // 3. Fetch initial snapshot via REST
    fetch_snapshot(rest_url.clone(), sender.clone()).await?;

    // println!("After snapshot");

    // println!("Reading from receiver");
    process_events(ticker, rest_url, sender.clone(), receiver).await?;
    Ok(())
}
