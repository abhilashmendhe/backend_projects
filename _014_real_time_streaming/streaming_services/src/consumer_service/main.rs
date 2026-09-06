use clap::Parser;
use tracing::{error, level_filters::LevelFilter};

use crate::{
    start_server_streaming::start_server_streaming,
    stream_service::message_stream_client::MessageStreamClient,
    utils::{app_data::ConsumerAppState, errors::ConsumerErr},
};

pub mod start_server_streaming;
pub mod utils;

pub mod stream_service {
    tonic::include_proto!("stream_service");
}
/*
    To run:
    $ cargo run --bin consumer_service -- --grpc-client-addr localhost --grpc-client-port 50051

    $ cargo watch  -q -c -w src/consumer_service -x "run --bin consumer_service -- --grpc-client-addr localhost --grpc-client-port 50051"
*/

#[derive(Debug, Parser)]
struct ServerCli {
    #[arg(long)]
    grpc_client_addr: String,

    #[arg(long)]
    grpc_client_port: u16,
}
#[tokio::main]
async fn main() -> Result<(), ConsumerErr> {
    // 1. Extract cli
    let scli = ServerCli::parse();
    let grpc_client_addr = scli.grpc_client_addr;
    let grpc_client_port = scli.grpc_client_port;

    // 2. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    tracing::info!("I am ingestion service");

    // 3. Configure grpc channel
    let grpc_full_addr = format!("http://{}:{}", grpc_client_addr, grpc_client_port);
    // println!("{}",grpc_full_addr);
    let grpc_channel = tonic::transport::Endpoint::from_shared(grpc_full_addr)?
        .connect()
        .await?;

    // 4. Create a client
    let client = MessageStreamClient::new(grpc_channel);

    // 5. Configure config and app state
    let consumer_app_state = ConsumerAppState::new(client);

    println!("Ok before start_server_streaming");
    // 6. Start streaming echo
    let _ = match start_server_streaming(consumer_app_state).await {
        Ok(_) => {}
        Err(err) => {
            error!("{:?}", err);
        }
    };
    // println!("Ok after start_server_streaming");
    Ok(())
}
