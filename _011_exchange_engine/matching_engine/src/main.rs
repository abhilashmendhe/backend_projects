use clap::Parser;
use matching_engine::{build_orderbook, utils::error::ExchangeErr};

/*
To run:
$ cargo run -- --ticker BTCUSDT --snapshot-limit 200
*/

#[derive(Debug, Parser)]
pub struct ServerCli {
    #[arg(long)]
    ticker: String,

    #[arg(long, default_value_t = 100)]
    timeunit: usize,

    #[arg(long, default_value_t = 5)]
    connection_timeout: u64,

    #[arg(long, default_value_t = 5000)]
    snapshot_limit: u64,
}

#[tokio::main]
async fn main() -> Result<(), ExchangeErr> {
    // 1. parse cli
    let scli = ServerCli::parse();
    let ticker = scli.ticker;
    let timeunit = scli.timeunit;
    let connection_timeout = scli.connection_timeout;
    let snapshot_limit = scli.snapshot_limit;
    let rest_url = format!(
        "https://api.binance.com/api/v3/depth?symbol={}&limit={}",
        ticker, snapshot_limit
    );
    let ws_url = format!(
        "wss://stream.binance.com:9443/ws/{}@depth@{}ms",
        ticker.to_ascii_lowercase(),
        timeunit
    );
    build_orderbook(ticker, rest_url, ws_url, connection_timeout).await?;
    Ok(())
}
