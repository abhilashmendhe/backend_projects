use std::env::VarError;

use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationWorkerErr {

    #[error("{}", 0)]
    VarError(#[from] VarError),

    #[error("{}", 0)]
    RedisErr(#[from] RedisError),
}