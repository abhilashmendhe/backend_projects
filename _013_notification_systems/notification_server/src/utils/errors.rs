use std::{env::VarError, num::ParseIntError};

use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationServerErr {
    #[error("{}", 0)]
    IOErr(#[from] std::io::Error),

    #[error("{}", 0)]
    EnvVarErr(#[from] VarError),

    #[error("{}", 0)]
    RedisErr(#[from] RedisError),

    #[error("{}", 0)]
    ParseIntErr(#[from] ParseIntError),
}
