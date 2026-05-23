use alloy::{
    hex::FromHexError,
    transports::{RpcError, TransportErrorKind},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletInfoErr {
    #[error("{}", .0)]
    EnvVarErr(#[from] std::env::VarError),

    #[error("{}", .0)]
    NoWallets(String),

    #[error("{}", .0)]
    RpcErr(#[from] RpcError<TransportErrorKind>),

    #[error("{}",.0)]
    FromHexErr(#[from] FromHexError),
}
