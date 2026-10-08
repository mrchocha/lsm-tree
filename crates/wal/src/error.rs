use std::io;

use common::bytes_reader::ByteReaderError;

#[derive(Debug)]
pub enum WalError {
    InvalidOperationType(u8),
    ByteReaderError(ByteReaderError),
    IoError(io::Error),
    Message(String),
}

impl From<ByteReaderError> for WalError {
    fn from(err: ByteReaderError) -> Self {
        WalError::ByteReaderError(err)
    }
}

impl From<io::Error> for WalError {
    fn from(err: io::Error) -> Self {
        WalError::IoError(err)
    }
}

impl From<&str> for WalError {
    fn from(err: &str) -> Self {
        WalError::Message(err.to_string())
    }
}
