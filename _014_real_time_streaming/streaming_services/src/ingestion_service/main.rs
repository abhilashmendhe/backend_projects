use actix_web::web;
use clap::Parser;
use tracing::level_filters::LevelFilter;

use crate::{
    start_server::start_server,
    stream_service::message_stream_client::MessageStreamClient,
    utils::{app_state::AppState, config::Config, errors::IngestionServiceErr},
};

pub mod models;
pub mod routes;
pub mod start_server;
pub mod utils;
pub mod view_router;

pub mod stream_service {
    tonic::include_proto!("stream_service");
}
/*
    To run:
    $ cargo run --bin ingestion_service -- --addr localhost --port 8080 --grpc-addr localhost --grpc-port 50051

    $ cargo watch  -q -c -w src/ingestion_service -x "run --bin ingestion_service -- --addr localhost --port 8080 --grpc-addr localhost --grpc-port 50051"
*/

#[derive(Debug, Parser)]
struct ServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long)]
    port: u16,

    #[arg(short, long, default_value_t = 4)]
    server_workers: usize,

    #[arg(long)]
    grpc_addr: String,

    #[arg(long)]
    grpc_port: u16,
}

#[actix_web::main]
async fn main() -> Result<(), IngestionServiceErr> {
    // 1. Parse command line
    let scli = ServerCli::parse();
    let server_addr = scli.addr;
    let port = scli.port;
    let server_workers = scli.server_workers;
    let grpc_addr = scli.grpc_addr;
    let grpc_port = scli.grpc_port;

    // 2. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    tracing::info!("I am ingestion service");

    // 3. Configure grpc channel
    let grpc_full_addr = format!("http://{}:{}", grpc_addr, grpc_port);
    // println!("{}",grpc_full_addr);
    let grpc_channel = tonic::transport::Endpoint::from_shared(grpc_full_addr)?
        .connect()
        .await?;

    // 4. Create a client
    let client = MessageStreamClient::new(grpc_channel);

    // 5. Configure config and app state
    let config = Config::new();
    let app_state = web::Data::new(AppState::new(config, client));

    // 6. Spin a actix server
    start_server(server_workers, &server_addr, port, app_state).await?;

    Ok(())
}
