use _2_queue_server::{
    models::payload_req::PayloadReq,
    run,
    utils::{app_state::AppState, errors::QueueServerErr},
};
use actix_web::web;
use clap::Parser;
use tracing::level_filters::LevelFilter;

/**
 * To run: cargo watch -q -c -w src/ -x "run -- --addr localhost -p 58233 --num-acx-servers=8 --num-process-workers=10 --channel-buffer-size=100"
 */

#[derive(Parser, Debug)]
pub struct ServerCli {
    #[arg(short, long)]
    addr: String,

    #[arg(short, long, default_value_t = 58233)]
    port: u16,

    #[arg(long, default_value_t = 4)]
    num_acx_servers: usize,

    #[arg(long, default_value_t = 4)]
    num_process_workers: usize,

    #[arg(long, default_value_t = 10)]
    channel_buffer_size: usize,
}

#[actix_web::main]
async fn main() -> Result<(), QueueServerErr> {
    // 0. Start tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    // 1. Get all the command arguments
    let server_cli = ServerCli::parse();
    let addr = &server_cli.addr;
    let port = server_cli.port;
    let num_acx_servers = server_cli.num_acx_servers;
    let num_process_workers = server_cli.num_process_workers;
    let channel_bufer_size = server_cli.channel_buffer_size;

    // 2. Create tokio channels
    let (tx, rx) = tokio::sync::mpsc::channel(channel_bufer_size);

    // 3. Create state
    let app_state = web::Data::new(AppState::new(tx));

    // 4. Run
    run(
        addr,
        port,
        num_acx_servers,
        num_process_workers,
        rx,
        app_state,
    )
    .await?;
    Ok(())
}
