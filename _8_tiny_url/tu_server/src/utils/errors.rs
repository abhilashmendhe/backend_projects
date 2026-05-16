use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TinyUrlError {
    #[error("{}", .0)]
    EnvVarErr(#[from] VarError),

    #[error("{}", .0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", .0)]
    IOErr(#[from] std::io::Error),
}
