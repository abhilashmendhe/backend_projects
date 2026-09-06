use std::net::SocketAddr;

use clap::Parser;
use tokio::{signal, sync::Mutex};
use tonic::transport::Server;
use tracing::{debug, info, level_filters::LevelFilter};

use crate::{
    create_server::{stream_server::PublishRequest, stream_server_service},
    utils::{app_data::StreamAppData, errors::StreamServerErr, logger::WalLogger},
};

pub mod create_server;
pub mod utils;

pub mod stream_service {
    tonic::include_proto!("stream_service");
}

/*
    $ cargo run --bin stream_queue -- --addr [::1] --port 50051 --aof-folder-path ./wal-logs

    $ cargo watch  -q -c -w src/stream_queue -x "run --bin stream_queue -- --addr [::1] --port 50051 --aof-folder-path ./wal-logs"
*/

#[derive(Debug, Parser)]
struct StreamServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long)]
    port: u16,

    #[arg(long, default_value = "./src/stream_queue/wal-logs")]
    aof_folder_path: String,

    #[arg(long, default_value_t = 1000)]
    channel_buff_size: usize,
}

#[tokio::main]
async fn main() -> Result<(), StreamServerErr> {
    // 1. Parse cli
    let sscli = StreamServerCli::parse();
    let addr = sscli.addr;
    let port = sscli.port;
    let aof_folder_path = sscli.aof_folder_path;
    let channel_buff_size = sscli.channel_buff_size;

    // 2. Create socket addr for grpc server, and channel
    let full_addr = format!("{}:{}", addr, port).parse::<SocketAddr>()?;
    let (tx, rx) = tokio::sync::mpsc::channel::<(u64, PublishRequest)>(channel_buff_size);

    // 3. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    debug!("I am stream server queue");
    info!("gRPC server starting on {}", full_addr);

    // 4. Init wal logger
    let wal_logger = WalLogger::new(aof_folder_path)?;

    // 5. Init app data
    let app_data = Mutex::new(StreamAppData::new(wal_logger));

    // 6. Start grpc server
    Server::builder()
        .add_service(stream_server_service(app_data, tx, Mutex::new(rx)))
        .serve_with_shutdown(full_addr, shutdown_signal())
        .await?;
    Ok(())
}

/// A future that completes when the application receives a termination signal.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait until either Ctrl+C or a SIGTERM is intercepted
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
