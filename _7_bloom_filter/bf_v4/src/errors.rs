use thiserror::Error;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

use crate::bf::BFCommand;

#[derive(Debug, Error)]
pub enum BFError {
    #[error("{}", .0)]
    SendErrorCommand(#[from] SendError<BFCommand>),

    #[error("{}", .0)]
    SendErrorQuery(#[from] SendError<bool>),

    #[error("{}", .0)]
    JoinError(#[from] JoinError),

    #[error("{}", .0)]
    OneshotRecvErr(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("{}", .0)]
    FileSaveError(#[from] std::io::Error),
}
