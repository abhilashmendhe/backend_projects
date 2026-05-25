use std::env::VarError;

use alloy::transports::{RpcError, TransportErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenIndexerErr {
    
    #[error("{}", .0)]
    EnvVarReadErr(#[from] VarError),

    #[error("{}", .0)]
    RpcConnectError(#[from] RpcError<TransportErrorKind>),
}