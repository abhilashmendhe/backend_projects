use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;

pub async fn start_streaming(url: String, connection_timeout: Duration) {
    match timeout(connection_timeout, connect_async(url)).await {
        Ok(Ok((ws_stream, response))) => {
            // println!("{:?}", response);
            println!("{}", response.status());

            for (ref header, _value) in response.headers() {
                println!("* {}", header);
            }

            let (_, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => println!("Received a message: {}", msg),
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
}
