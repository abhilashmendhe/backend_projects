use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExchangeErr {
    #[error("REST Err: {0}")]
    RESTErr(#[from] reqwest::Error),

    #[error("{0}")]
    HttpResponseErr(String),
}
