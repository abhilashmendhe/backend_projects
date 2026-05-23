use alloy::{primitives::Address, providers::ProviderBuilder};
use std::str::FromStr;

use crate::{services::fetch_wallet_info::wallet_info, utils::error::WalletInfoErr};

pub mod services;
pub mod utils;

pub async fn run(rpc_url: String, wallets: Vec<String>) -> Result<(), WalletInfoErr> {
    // 1. Build provider
    let provider = ProviderBuilder::new().connect(&rpc_url).await?;

    for wallet in wallets {
        println!("wallet addr: {}", wallet);
        let wallet_addr = Address::from_str(&wallet)?;
        // println!("{:?}",  wallet_addr);
        wallet_info(&provider, wallet_addr).await;
    }
    Ok(())
}
