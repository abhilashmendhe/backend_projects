use serde::de;
use std::num::ParseFloatError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExchangeErr {
    #[error("REST Err: {0}")]
    RESTErr(#[from] reqwest::Error),

    #[error("Serde parsing failed: {0}")]
    SerdeErrCustom(String),

    #[error("Parse float error: {0}")]
    ParseFloatErr(#[from] ParseFloatError),

    #[error("Serde JSON error: {0}")]
    SerdeJSONErr(#[from] serde_json::error::Error),

    #[error("REST Response Err: {0}")]
    HttpResponseErr(String),
}

impl de::Error for ExchangeErr {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        ExchangeErr::SerdeErrCustom(msg.to_string())
    }
}
