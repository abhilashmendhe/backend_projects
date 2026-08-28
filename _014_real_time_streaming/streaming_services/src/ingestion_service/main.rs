use clap::Parser;
use tracing::level_filters::LevelFilter;

use crate::{start_server::start_server, utils::errors::IngestionServiceErr};

pub mod start_server;
pub mod utils;

/*
    To run:
    $ cargo run --bin ingestion_service -- --addr localhost --port 8000
*/

#[derive(Debug, Parser)]
struct ServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long)]
    port: u16,

    #[arg(short, long, default_value_t = 4)]
    server_workers: usize,
}

#[actix_web::main]
async fn main() -> Result<(), IngestionServiceErr> {
    println!("I am ingestion service!");

    // 1. Parse command line
    let scli = ServerCli::parse();
    let server_addr = scli.addr;
    let port = scli.port;
    let server_workers = scli.server_workers;

    // 2. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // 3. Spin a actix server
    start_server(server_workers, &server_addr, port).await?;

    Ok(())
}
