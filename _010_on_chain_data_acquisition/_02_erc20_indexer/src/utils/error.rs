use std::env::VarError;

use alloy::transports::{RpcError, TransportErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenIndexerErr {
    #[error("{}", .0)]
    EnvVarReadErr(#[from] VarError),

    #[error("{}", .0)]
    RpcConnectError(#[from] RpcError<TransportErrorKind>),
    // #[error("{}", .0)]
    // FromAlloyErr(#[from] Error),
    #[error("{}", .0)]
    BlockNotFound(String),

    #[error("{}", .0)]
    LogNotFound(String),

    #[error("{}", .0)]
    FailedToDecodeLog(String),
}
