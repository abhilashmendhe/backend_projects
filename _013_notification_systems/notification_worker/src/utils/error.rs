use std::{env::VarError, num::ParseIntError};

use redis::RedisError;
use reqwest::header::ToStrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotificationWorkerErr {
    #[error("{}", 0)]
    VarError(#[from] VarError),

    #[error("{}", 0)]
    RedisErr(#[from] RedisError),

    #[error("{}", 0)]
    ParseIntErr(#[from] ParseIntError),

    #[error("{}", 0)]
    ToStrErr(#[from] ToStrError),

    #[error("{}", 0)]
    SQLErr(#[from] sqlx::Error),

    #[error("{}", 0)]
    ReqwestErr(#[from] reqwest::Error),
}
