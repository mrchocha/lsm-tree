use tokio::{
    fs::File,
    io::{AsyncSeekExt, BufReader, SeekFrom},
};

use crate::{error::WalError, file::WalFile, manager::WalOptions, record::WalRecord, util};

pub struct WalIterator<'a> {
    pub seq_no: u32,

    file: WalFile,
    reader: BufReader<File>,

    options: &'a WalOptions,
}

impl<'a> WalIterator<'a> {
    const HEADER_SIZE: u64 = 4;

    pub async fn new(options: &'a WalOptions) -> Result<Self, WalError> {
        let first_file = util::get_first_file(options, None)
            .await?
            .expect("Not able to find WAL files");

        let file_fd = first_file.get_file_fd().await?;
        let mut reader = BufReader::new(file_fd);
        reader.seek(SeekFrom::Start(Self::HEADER_SIZE)).await?;

        Ok(Self {
            seq_no: first_file.seq_no,
            file: first_file,
            reader,
            options,
        })
    }

    pub async fn has_next(&mut self) -> Result<bool, WalError> {
        let size = self.file.get_size();
        let offset = self.reader.stream_position().await? as usize;

        if offset < size {
            return Ok(true);
        }

        if let Some(next_file) = util::get_first_file(self.options, Some(self.seq_no + 1)).await? {
            self.seq_no = self.file.seq_no;

            let file_fd = next_file.get_file_fd().await?;
            let mut reader = BufReader::new(file_fd);
            reader.seek(SeekFrom::Start(Self::HEADER_SIZE)).await?;

            self.file = next_file;
            self.reader = reader;

            return Ok((self.reader.stream_position().await? as usize) < self.file.get_size());
        }

        Ok(false)
    }

    pub async fn next(&mut self) -> Result<Option<WalRecord>, WalError> {
        if !self.has_next().await? {
            return Ok(None);
        }

        let wal_record = WalRecord::from_buffer(&mut self.reader).await?;

        Ok(Some(wal_record))
    }
}
