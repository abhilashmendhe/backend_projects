use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let uri = "wss://stream.binance.us:9443/ws/btcusdt@depth@100ms";
    let (ws_stream, response) = connect_async(uri).await.unwrap();
    println!("{}", response.status());

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }

    println!("hi");
}
