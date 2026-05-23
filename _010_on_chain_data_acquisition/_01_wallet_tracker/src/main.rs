use _01_wallet_tracker::{run, utils::error::WalletInfoErr};
use clap::Parser;

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
    let api_key = std::env::var("RPC_API_KEY")?;

    // 2. create full rpc url
    let full_rpc_url = format!("{}/{}", rpc_url, api_key);

    // 3. run
    run(full_rpc_url, wallets).await
}
