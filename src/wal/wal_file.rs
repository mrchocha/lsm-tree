use crate::wal::wal_record::{self, WalRecord};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::path::PathBuf;

const WAL_FILE_PATH: &str = "./wal";

pub struct WalFileHeader {
    pub seq_no: u32,
}

impl WalFileHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4);

        bytes.extend_from_slice(&self.seq_no.to_be_bytes());

        bytes
    }
}

pub struct WalFile {
    pub name: String,
    pub path: PathBuf,
    pub header: WalFileHeader,

    file: File,
}

impl WalFile {
    pub fn create(seq_no: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let name: String = format!("{seq_no}_data.wal");
        let path = PathBuf::from(WAL_FILE_PATH).join(&name);

        let header = WalFileHeader { seq_no };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&path)?;

        file.write_all(&header.to_bytes())?;

        Ok(Self {
            name,
            path,
            header,
            file,
        })
    }

    pub fn from_path(str_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(str_path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;

        let mut reader = BufReader::new(&file);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        let header = WalFileHeader {
            seq_no: u32::from_be_bytes(buffer),
        };

        let name = path
            .file_name()
            .ok_or("path has no filename")?
            .to_str()
            .ok_or("filename is not valid UTF-8")?
            .to_string();

        Ok(WalFile {
            name,
            path: path.to_path_buf(),
            header,
            file,
        })
    }

    pub fn get_size(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let metadata = self.file.metadata()?;
        let file_size = metadata.len();

        Ok(file_size)
    }

    pub fn reset(&mut self, seq_no: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.file.set_len(0)?;

        self.file.seek(SeekFrom::Start(0))?;

        self.header = WalFileHeader { seq_no };
        self.file.write_all(&self.header.to_bytes())?;

        Ok(())
    }

    pub fn append(&mut self, wal_record: &WalRecord) -> Result<(), Box<dyn std::error::Error>> {
        self.file.write_all(&wal_record.to_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create() {}
}
