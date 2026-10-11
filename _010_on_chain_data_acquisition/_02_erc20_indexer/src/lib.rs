use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};

use crate::utils::error::TokenIndexerErr;

pub mod services;
pub mod utils;

pub async fn run(rpc_url: &str, _block_num_tag: BlockNumberOrTag) -> Result<(), TokenIndexerErr> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    // provider
    // Get the latest block number.

    if let Some(latest_block) = provider.get_block(BlockId::latest()).await? {
        // println!("Block number: {}", latest_block.number());
        // println!("{:#?}", latest_block);
        let latest_block_number = latest_block.number();

        let header = latest_block.header;
        println!("Header:\n");
        println!("{:#?}", header);

        let uncles = latest_block.uncles;
        println!("Uncles:\n");
        println!("{:#?}", uncles);

        let transaction = latest_block.transactions;
        println!("Transactions:\n");
        println!("{:#?}", transaction);

        let withdrawals = latest_block.withdrawals;
        println!("Withdrawals:\n");
        println!("{:#?}", withdrawals);

        // Create a filter to get all logs from the latest block.
        let filter = Filter::new().from_block(latest_block_number);
        // Get all logs from the latest block that match the filter.
        let logs = provider.get_logs(&filter).await?;
        println!("Total logs: {}", logs.len());
        for log in logs {
            // println!("{:?}\n", log);
        }
    }

    Ok(())
}
