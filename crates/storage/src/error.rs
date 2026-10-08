use std::io;

use wal::error::WalError;

#[derive(Debug)]
pub enum StoreError {
    WalError(WalError),
    IoError(io::Error),
    Message(String),
    Other(Box<dyn std::error::Error>),
}

impl From<WalError> for StoreError {
    fn from(err: WalError) -> Self {
        StoreError::WalError(err)
    }
}

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> Self {
        StoreError::IoError(err)
    }
}

impl From<&str> for StoreError {
    fn from(err: &str) -> Self {
        StoreError::Message(err.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for StoreError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        StoreError::Other(err)
    }
}
