use std::io::{BufReader, Read};
use std::{fs::OpenOptions, path::Path};

pub struct SSTFile {
    seq_no: u32,
}

impl SSTFile {
    pub fn from_path(str_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(str_path);

        let file = OpenOptions::new().read(true).append(true).open(path)?;

        let mut reader = BufReader::new(&file);

        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        Ok(Self {
            seq_no: u32::from_be_bytes(buffer),
        })
    }
}
