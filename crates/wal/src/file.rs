use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::error::WalError;
use crate::manager::WalOptions;
use crate::record::WalRecord;

pub struct WalFile {
    pub seq_no: u32,
    pub name: String,
    pub path: PathBuf,

    file_writer: BufWriter<File>,
    size: usize,
    buffer: Vec<u8>,
}

impl WalFile {
    pub async fn create(options: &WalOptions, seq_no: u32) -> Result<Self, WalError> {
        let name: String = format!("{:020}.wal", seq_no);
        let path = PathBuf::from(options.file_path.clone()).join(&name);

        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)
            .await?;

        let mut writer = BufWriter::new(file);

        writer.write_all(&seq_no.to_be_bytes()).await?;
        writer.flush().await?;

        Ok(Self {
            name,
            path,
            seq_no,
            file_writer: writer,
            size: 4, // with seq no
            buffer: Vec::new(),
        })
    }

    pub async fn from_path(str_path: &str) -> Result<Self, WalError> {
        let path = Path::new(str_path);

        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(path)
            .await?;

        let reader_file = file.try_clone().await?;
        let mut reader = BufReader::new(reader_file);

        let writer = BufWriter::new(file);

        let metadata = writer.get_ref().metadata().await?;
        let file_size = metadata.len() as usize;

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer).await?;

        let seq_no = u32::from_be_bytes(buffer);

        let name = path
            .file_name()
            .ok_or("path has no filename")?
            .to_str()
            .ok_or("filename is not valid UTF-8")?
            .to_string();

        Ok(WalFile {
            name,
            path: path.to_path_buf(),
            seq_no,
            file_writer: writer,
            buffer: Vec::new(),
            size: file_size,
        })
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub async fn get_file_fd(&self) -> Result<File, WalError> {
        Ok(self.file_writer.get_ref().try_clone().await?)
    }

    pub async fn append(&mut self, wal_record: &WalRecord) -> Result<(), WalError> {
        let bytes = wal_record.to_bytes();
        self.file_writer.write_all(&bytes).await?;
        self.size += bytes.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create() {}
}
