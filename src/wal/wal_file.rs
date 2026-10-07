use crate::wal::wal_manager::WalOptions;
use crate::wal::wal_record::WalError;
use crate::wal::wal_record::WalRecord;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::path::PathBuf;

pub struct WalFile {
    pub seq_no: u32,
    pub name: String,
    pub path: PathBuf,

    file: File,
}

impl WalFile {
    pub fn create(options: &WalOptions, seq_no: u32) -> Result<Self, WalError> {
        let name: String = format!("{:020}.wal", seq_no);
        let path = PathBuf::from(options.file_path.clone()).join(&name);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&path)?;

        file.write_all(&seq_no.to_be_bytes())?;

        Ok(Self {
            name,
            path,
            seq_no,
            file,
        })
    }

    pub fn from_path(str_path: &str) -> Result<Self, WalError> {
        let path = Path::new(str_path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;

        let mut reader = BufReader::new(&file);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

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
            file,
        })
    }

    pub fn get_size(&self) -> Result<u64, WalError> {
        let metadata = self.file.metadata()?;
        let file_size = metadata.len();

        Ok(file_size)
    }

    pub fn reset(&mut self, seq_no: u32) -> Result<(), WalError> {
        self.file.set_len(0)?;

        self.file.seek(SeekFrom::Start(0))?;

        self.file.write_all(&seq_no.to_be_bytes())?;

        Ok(())
    }

    pub fn append(&mut self, wal_record: &WalRecord) -> Result<(), WalError> {
        self.file.lock();
        self.file.write_all(&wal_record.to_bytes())?;
        self.file.unlock();
        Ok(())
    }

    pub fn get_file_fd(&self) -> File {
        self.file.try_clone().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create() {}
}
