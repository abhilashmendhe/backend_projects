use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let uri = "wss://stream.binance.us:9443/ws/btcusdt@trade";
    let (ws_stream, response) = connect_async(uri).await.unwrap();
    println!("{}", response.status());
}
