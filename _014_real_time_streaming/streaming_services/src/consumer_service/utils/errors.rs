use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerErr {
    #[error("{}", 0)]
    IoErr(#[from] std::io::Error),

    #[error("{}", 0)]
    AddrParseErr(#[from] std::net::AddrParseError),

    #[error("{}", 0)]
    TonicTransportErr(#[from] tonic::transport::Error),
}
