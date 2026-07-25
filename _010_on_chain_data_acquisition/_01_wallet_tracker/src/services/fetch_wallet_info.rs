use alloy::{primitives::{Address, B256}, providers::Provider, rpc::types::Filter, sol};
use std::str::FromStr;
use alloy::primitives::{address, keccak256};

use crate::WalletInfoErr;

// sol! {
//     #[sol(rpc)]
//     interface IERC20 {
//         function balanceOf(address) external view returns (uint256);
//     }
// }

pub async fn wallet_info(provider: impl Provider, w_address: Address) -> Result<(), WalletInfoErr> {
    // let trie_acc_info = provider.get_account(w_address).await;

    // println!("{:#?}", trie_acc_info);

    // let code = provider.get_code_at(w_address).await?;

    // if code.is_empty() {
    //     println!("It is an EOA.");
    // } else {
    //     println!("It is a smart contract");
    // }

    // let transfer_sig = keccak256("Transfer(address,address,uint256)".as_bytes());
    // let wallet_topic: B256 = w_address.into_word();

    // let incoming = Filter::new()
    //     .event_signature(transfer_sig)
    //     .topic2(wallet_topic);
        // .block
        // .block_option();
    //     .from_block(24982129)
    // .to_block(24982135);


    ////////////
    let wallet =
    address!("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

// USDC contract
let usdc =
    address!("A0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");

let transfer_sig =
    keccak256("Transfer(address,address,uint256)".as_bytes());

let wallet_topic: B256 = wallet.into_word();

let filter = Filter::new()
    .address(usdc)
    .event_signature(transfer_sig)
    .topic2(wallet_topic);
// incoming transfers
    /// //////////////////

    let logs = provider.get_logs(&filter).await?;
    println!("{:?}",logs);
    // for log in logs {
    //     println!("{:?}",log);
    // }

    // println!("{}")
    // let usdc_addr = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?;
    // // Filter::new()
    // let token = IERC20::new(usdc_addr, &provider);
    // let balance = token.balanceOf(w_address).call().await.unwrap();
    // println!("balance: {}", balance);
    Ok(())
}
