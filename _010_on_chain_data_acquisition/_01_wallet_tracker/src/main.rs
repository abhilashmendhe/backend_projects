use _01_wallet_tracker::{run, utils::error::WalletInfoErr};
use clap::Parser;

/*
*  $ cargo run -- --rpc-url https://eth-mainnet.g.alchemy.com/v2 --wallets 0x165CD37b4C644C2921454429E7F9358d18A45e14
*  $ cargo run -- --rpc-url https://rpc.ankr.com/eth --wallets 0x165CD37b4C644C2921454429E7F9358d18A45e14
*/

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(short, long)]
    wallets: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), WalletInfoErr> {
    // 0. Get server clie
    let scli = ServerCli::parse();
    let rpc_url = scli.rpc_url;
    let wallets = scli.wallets;

    if wallets.len() < 1 {
        return Err(WalletInfoErr::NoWallets(
            "Need to pass atleast one wallet address to fetch information!".to_string(),
        ));
    }

    // 1. read API KEY from .env file
    dotenv::dotenv().ok();
    let alch_api_key = std::env::var("ALCHEMY_RPC_API_KEY")?;
    let ankr_api_key = std::env::var("ANKR_RPC_API_KEY")?;

    let api_key = if rpc_url.contains("ankr") {
        ankr_api_key
    } else {
        alch_api_key
    };

    // 2. create full rpc url
    let full_rpc_url = format!("{}/{}", rpc_url, api_key);

    // 3. run
    run(full_rpc_url, wallets).await
}
