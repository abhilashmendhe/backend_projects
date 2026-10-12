use std::io::{self, Write};

use _02_erc20_indexer::{run, utils::error::TokenIndexerErr};
use alloy::eips::BlockNumberOrTag;
use clap::Parser;

/*
*  $ cargo run -- --rpc-url https://eth-sepolia.g.alchemy.com/v2
*  $ cargo run -- --rpc-url https://eth-mainnet.g.alchemy.com/v2
*  $ cargo run -- --rpc-url https://rpc.ankr.com/eth
   $ cargo run -- --rpc-url https://eth-mainnet.g.alchemy.com/v2 --block-num-or-tag 25170558
   ----------------- wss ------------------
   $ cargo run -- --rpc-url wss://eth-mainnet.g.alchemy.com/v2
*/

#[derive(Parser, Debug)]
struct ServerCli {
    #[arg(long)]
    rpc_type: String,

    #[arg(long, default_value = "")]
    rpc_url: String,

    #[arg(long, default_value = "latest")]
    block_num_or_tag: BlockNumberOrTag,
}

#[tokio::main]
async fn main() -> Result<(), TokenIndexerErr> {
    // 1. Parse command line args
    let scli = ServerCli::parse();
    let rpc_type = &scli.rpc_type;

    let block_num_or_tag = scli.block_num_or_tag;

    // 2. read .env variables
    let _ = dotenv::dotenv().ok();
    let alch_api_key = std::env::var("ALCHEMY_RPC_API_KEY")?;
    let ankr_api_key = std::env::var("ANKR_RPC_API_KEY")?;
    let infura_api_key = std::env::var("INFURA_RPC_API_KEY")?;

    // 4. create full rpc url
    let full_rpc_url = if rpc_type == "ws" {
        let rpc_ws_list = [
            "wss://ethereum-rpc.publicnode.com",
            "wss://eth.drpc.org",
            "wss://ethereum-mainnet.gateway.tatum.io",
            "wss://ethereum-sepolia-rpc.publicnode.com",
            "wss://ethereum-sepolia-gateway.tatum.io",
            "wss://mainnet.infura.io/ws/v3",
            "wss://sepolia.infura.io/ws/v3",
        ];
        for i in 0..rpc_ws_list.len() {
            println!("{}. {}", i + 1, rpc_ws_list[i]);
        }
        print!("Select ws rpc number > ");
        io::stdout().flush().unwrap();

        let mut rpc_ws_num = String::new();
        io::stdin().read_line(&mut rpc_ws_num)?;

        let rpc_ws_num = rpc_ws_num
            .trim()
            .parse::<usize>()
            .expect("Number should be greater than 0");

        if rpc_ws_num > rpc_ws_list.len() {
            panic!("Invalid rpc selection");
        }
        let rpc_url = rpc_ws_list[rpc_ws_num - 1];
        if rpc_url.contains("infura") {
            format!("{}/{}", rpc_url, infura_api_key)
        } else {
            rpc_url.to_string()
        }
    } else if rpc_type == "http" {
        let rpc_url = &scli.rpc_url;
        if rpc_url.len() == 0 {
            panic!("Please provide the HTTP rpc link via --rpc-url");
        }
        let api_key = if rpc_url.contains("ankr") {
            ankr_api_key
        } else if rpc_url.contains("alchemy") {
            alch_api_key
        } else {
            panic!("Please check with the rpc provider");
        };
        format!("{}/{}", rpc_url, api_key)
    } else {
        panic!("Please pass 'http' or 'ws' rpc type.");
    };

    // 5. call run function
    run(&full_rpc_url, &rpc_type, block_num_or_tag).await?;

    Ok(())
}
