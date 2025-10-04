use thiserror::Error;

#[derive(Debug, Error)]
pub enum KVError {

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error)
}