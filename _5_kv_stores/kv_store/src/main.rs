/*
To run:
    $ cargo watch -q -c -w src/ -x run
*/

use kv_store::{run, utils::{app_state::AppState, errors::KVError}};
use clap::Parser;
use tracing::level_filters::LevelFilter;


#[derive(Parser)]
#[command(name = "KV-Store")]
// #[command(version = "1.0")]
#[command(about = "Timebased Key-Value Store Server", long_about = None)]
struct Args {

    /// IP Address
    #[arg(long, default_value="0.0.0.0")]
    ip_addr: String,

    /// Port
    #[arg(long, default_value="8000")]
    port: String
}

#[tokio::main]
async fn main() -> Result<(), KVError> {
    
    // 1. enable tracing.
    tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

    // 2. Get address and port from cmd
    let a = Args::parse();

    let address = &a.ip_addr;
    let port = &a.port;
    let addr = format!("{}:{}", address, port);

    // 3. Initialize state
    let app_state = AppState::new();

    run(app_state, &addr).await?;

    Ok(())
}
