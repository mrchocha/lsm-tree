use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct WalFileHeader {
    seq_no: u32,
}

pub struct WalFile {
    name: String,
    path: String,
    size: u64,
    header: WalFileHeader,
}

impl WalFile {
    pub fn from_path(str_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(str_path);

        let file = File::open(path)?;
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

        let metadata = file.metadata()?;
        let file_size = metadata.len();

        Ok(WalFile {
            name,
            path: str_path.to_string(),
            size: file_size,
            header,
        })
    }
}
