use alloy::providers::ProviderBuilder;

use crate::utils::error::TokenIndexerErr;

pub mod services;
pub mod utils;


pub async fn run(rpc_url: &str) -> Result<(), TokenIndexerErr> { 

    let provider = ProviderBuilder::new()
            .connect(rpc_url)
            .await?;

    
    Ok(())
}