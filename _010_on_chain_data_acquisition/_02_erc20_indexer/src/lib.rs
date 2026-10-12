use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    providers::{Provider, ProviderBuilder, WsConnect},
};

use crate::utils::error::TokenIndexerErr;
use futures_util::StreamExt;

pub mod services;
pub mod utils;

pub async fn run(
    rpc_url: &str,
    rpc_type: &str,
    _block_num_tag: BlockNumberOrTag,
) -> Result<(), TokenIndexerErr> {
    if rpc_type == "http" {
        let provider = ProviderBuilder::new().connect(rpc_url).await?;

        // provider
        // Get the latest block number.
        if let Some(latest_block) = provider.get_block(BlockId::latest()).await? {
            // println!("Block number: {}", latest_block.number());
            println!("{:#?}", latest_block);
            // let latest_block_number = latest_block.number();

            // let header = latest_block.header;
            // println!("Header:\n");
            // println!("{:#?}", header);

            // let uncles = latest_block.uncles;
            // println!("Uncles:\n");
            // println!("{:#?}", uncles);

            // let transaction = latest_block.transactions;
            // println!("Transactions:\n");
            // println!("{:#?}", transaction);

            // let withdrawals = latest_block.withdrawals;
            // println!("Withdrawals:\n");
            // println!("{:#?}", withdrawals);

            // // Create a filter to get all logs from the latest block.
            // let filter = Filter::new().from_block(latest_block_number);
            // // Get all logs from the latest block that match the filter.
            // let logs = provider.get_logs(&filter).await?;
            // println!("Total logs: {}", logs.len());
            // for log in logs {
            //     // println!("{:?}\n", log);
            // }
        }
    } else {
        // 1. Subscribe and create to provider
        let ws = WsConnect::new(rpc_url);
        let block_provider = ProviderBuilder::new().connect_ws(ws).await?;

        // 2. Subscriber to
        let block_subs = block_provider.subscribe_blocks().await?;
        let mut block_stream = block_subs.into_stream();

        while let Some(block) = block_stream.next().await {
            println!("{:?}\n", block);
        }
    }
    Ok(())
}
