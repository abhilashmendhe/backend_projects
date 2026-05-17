use bf_server::{
    run,
    utils::{app_state::AppState, error::BloomFilterErr},
};
use bloomfilter::bf::BloomFilter;
use clap::Parser;
use tracing::level_filters::LevelFilter;

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    localhost: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value_t = 4)]
    bf_server_workers: usize,
}

#[tokio::main]
async fn main() -> Result<(), BloomFilterErr> {
    // 0. Get args
    let scli = ServerCli::parse();
    let localhost = scli.localhost;
    let port = scli.port;
    let bf_server_workers = scli.bf_server_workers;

    // 1. Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    // 1. Start bloom filter
    let bloom_filter = BloomFilter::spawn(1000, 0.01, bf_server_workers, None).await?;

    // 2. Create app state
    let app_data = AppState::new(bloom_filter);

    // 2. Run axum server
    let addr = format!("{}:{}", localhost, port);
    run(&addr, app_data).await?;
    Ok(())
}
