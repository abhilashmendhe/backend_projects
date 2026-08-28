use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestionServiceErr {
    #[error("{}", 0)]
    IoErr(#[from] std::io::Error),
}
