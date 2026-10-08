use std::{
    fs::File,
    io::{self, BufWriter},
    sync::PoisonError,
};

use common::bytes_reader::ByteReaderError;

#[derive(Debug)]
pub enum WalError {
    InvalidOperationType(u8),
    ByteReaderError(ByteReaderError),
    IoError(io::Error),
    Message(String),
    LockPoisoned,
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

impl<'a> From<PoisonError<&'a mut BufWriter<File>>> for WalError {
    fn from(_err: PoisonError<&'a mut BufWriter<File>>) -> Self {
        WalError::LockPoisoned
    }
}
