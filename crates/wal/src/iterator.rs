use std::{fs::File, io::BufReader};

use crate::{
    error::WalError,
    file::WalFile,
    manager::{WalManager, WalOptions},
    record::WalRecord,
};

pub struct WalIterator<'a> {
    pub seq_no: u32,

    file: WalFile,
    reader: BufReader<File>,
    offset: u64,

    options: &'a WalOptions,
}

impl<'a> WalIterator<'a> {
    pub fn new(options: &'a WalOptions) -> Result<Self, WalError> {
        let first_file =
            WalManager::get_first_file(options, None)?.expect("Not able to find WAL files");

        let file_fd = first_file.get_file_fd();
        let mut reader = BufReader::new(file_fd);
        reader.seek_relative(4)?;

        Ok(Self {
            seq_no: first_file.seq_no,
            file: first_file,
            offset: 0,
            reader,
            options,
        })
    }

    pub fn has_next(&mut self) -> Result<bool, WalError> {
        let size = self.file.get_size()?;
        if self.offset < size {
            return Ok(true);
        }

        if self.offset >= size
            && let Some(next_file) =
                WalManager::get_first_file(self.options, Some(self.seq_no + 1))?
        {
            self.file = next_file;
            self.seq_no = self.file.seq_no;
            self.offset = 0;
            self.reader = BufReader::new(self.file.get_file_fd());
            self.reader.seek_relative(4)?;

            return Ok(true);
        }

        Ok(false)
    }

    pub fn next(&mut self) -> Result<Option<WalRecord>, WalError> {
        if !self.has_next()? {
            return Ok(None);
        }

        let wal_record = WalRecord::from_buffer(&mut self.reader)?;

        Ok(Some(wal_record))
    }
}
