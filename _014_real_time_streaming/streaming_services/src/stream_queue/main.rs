use std::net::SocketAddr;

use clap::Parser;
use tonic::transport::Server;
use tracing::{debug, info, level_filters::LevelFilter};

use crate::{create_server::stream_server_service, utils::{app_data::StreamAppData, errors::StreamServerErr}};

pub mod create_server;
pub mod utils;

pub mod stream_service {
    tonic::include_proto!("stream_service");
}

/*
    $ cargo run --bin stream_queue -- --addr [::1] --port 50051 --aof-folder-path ./wal-logs
*/

#[derive(Debug, Parser)]
struct StreamServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long)]
    port: u16,

    #[arg(long)]
    aof_folder_path: String 
}

#[tokio::main]
async fn main() -> Result<(), StreamServerErr> {
    // 1. Parse cli
    let sscli = StreamServerCli::parse();
    let addr = sscli.addr;
    let port = sscli.port;
    let aof_folder_path = sscli.aof_folder_path;

    // 2. Create socket addr for grpc server
    let full_addr = format!("{}:{}", addr, port).parse::<SocketAddr>()?;

    // 3. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    debug!("I am stream server queue");
    info!("gRPC server starting on {}", full_addr);

    // 4. Init app data
    let app_data = StreamAppData::new(aof_folder_path);

    // 5. Start grpc server
    Server::builder()
        .add_service(stream_server_service())
        .serve(full_addr)
        .await?;
    Ok(())
}
