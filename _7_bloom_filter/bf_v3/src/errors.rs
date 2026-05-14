use thiserror::Error;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use crate::BFCommand;

#[derive(Debug, Error)]
pub enum BFError {
    #[error("{}", .0)]
    SendError(#[from] SendError<BFCommand>),

    #[error("{}", .0)]
    JoinError(#[from] JoinError),

    #[error("{}", .0)]
    OneshotRecvErr(#[from] tokio::sync::oneshot::error::RecvError),
}
