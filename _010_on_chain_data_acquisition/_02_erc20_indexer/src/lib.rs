use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::utils::format_ether,
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockTransactions, Filter},
    sol,
};

// 1. Add this import to the top of your file
use alloy::consensus::Transaction as _;

use crate::utils::error::TokenIndexerErr;

pub mod services;
pub mod utils;

sol! {
    #[sol(rpc)]
    #[derive(Debug)]
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 value
    );


    #[sol(rpc)]
    #[derive(Debug)]
    interface IERC20 {
        function balanceOf(address) external view returns (uint256);
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
        function name() external view returns (string);
    }
}

pub async fn run(
    rpc_url: &str,
    block_number_or_tag: BlockNumberOrTag,
) -> Result<(), TokenIndexerErr> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;
    let block_id = BlockId::from(block_number_or_tag);
    // let block_id = BlockId::Number(BlockNumberOrTag::Number(25170558));

    let block_future = provider.get_block(block_id).full();

    let filter = Filter::new()
        .from_block(block_number_or_tag)
        .to_block(block_number_or_tag);

    let logs_future = provider.get_logs(&filter);

    let (block_res, logs_res) = tokio::join!(block_future, logs_future);

    let block = block_res?;
    if let Some(block) = block {
        println!("#------------------------- Transactions -------------------------#");
        match block.clone().transactions {
            BlockTransactions::Full(txs) => {
                for tx in txs {
                    let tx_hash = tx.inner.hash();
                    let from_addr = tx.inner.signer();
                    let to_addr = tx.inner.to();
                    let value = tx.inner.value();
                    let eth_value_wei = format_ether(value);
                    let gas_limit = tx.inner.gas_limit();
                    println!("Tx hash: {}", tx_hash);
                    println!("From addr: {}", from_addr);
                    println!("To addr: {:?}", to_addr);
                    println!("Value: {:?} wei", eth_value_wei);
                    println!("Gas limit: {:?}", gas_limit);
                    println!();
                    // break;
                }
            }
            BlockTransactions::Hashes(_txs) => {}
            BlockTransactions::Uncle => todo!(),
        }
    } else {
        eprintln!("Block id: {} not found", block_id);
    }

    let logs = logs_res.map_err(|_| TokenIndexerErr::LogNotFound("No logs found".to_string()))?;
    println!("#------------------------- Logs -------------------------#");
    for log in logs {
        // println!("{}", type_name_of_val(&log));
        // println!("{:?}", log);
        match log.log_decode::<Transfer>() {
            Ok(log_decode) => {
                let data = log_decode.data();
                println!("Token address: {}", log.address());
                println!("from: {:?}", data.from);
                println!("to: {:?}", data.to);
                println!("value: {:?}", data.value);
                println!("");
            }
            Err(_) => {
                // eprintln!("Filed to decode log!")
            }
        }
    }
    Ok(())
}
