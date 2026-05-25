use _02_erc20_indexer::{run, utils::error::TokenIndexerErr};
use alloy::eips::BlockNumberOrTag;
use clap::Parser;

/*
*  $ cargo run -- --rpc-url https://eth-mainnet.g.alchemy.com/v2
*  $ cargo run -- --rpc-url https://rpc.ankr.com/eth
   $ cargo run -- --rpc-url https://eth-mainnet.g.alchemy.com/v2 --block-num-or-tag 25170558
   ----------------- wss ------------------
   $ cargo run -- --rpc-url wss://eth-mainnet.g.alchemy.com/v2
*/

// enum

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(long, default_value = "latest")]
    block_num_or_tag: BlockNumberOrTag,
}

#[tokio::main]
async fn main() -> Result<(), TokenIndexerErr> {
    // 1. Parse command line args
    let scli = ServerCli::parse();
    let rpc_url = &scli.rpc_url;
    let block_num_or_tag = scli.block_num_or_tag;

    // 2. read .env variables
    let _ = dotenv::dotenv().ok();
    let alch_api_key = std::env::var("ALCHEMY_RPC_API_KEY")?;
    let ankr_api_key = std::env::var("ANKR_RPC_API_KEY")?;
    let api_key = if rpc_url.contains("ankr") {
        ankr_api_key
    } else {
        alch_api_key
    };

    // 3. create full rpc url
    let full_rpc_url = format!("{}/{}", rpc_url, api_key);
    // println!("{:?}",full_rpc_url);

    // 4. call run function
    run(&full_rpc_url, block_num_or_tag).await?;

    Ok(())
}
