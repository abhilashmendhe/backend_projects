use std::net::AddrParseError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StreamServerErr {

    #[error("{}", 0)]
    IoErr(#[from] std::io::Error),
    
    #[error("{}", 0)]
    AddrParseErr(#[from] AddrParseError),

    #[error("{}", 0)]
    TonicTransportErr(#[from] tonic::transport::Error),
}
