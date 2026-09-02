use std::{fs, io::Error};

use crate::wal::{
    wal_file::WalFile,
    wal_record::{WalError, WalRecord},
};

pub struct WalManager {
    current_file: Option<WalFile>,
    max_wal_file_seq_no: u32,
}

impl WalManager {
    fn new() -> Self {
        WalManager {
            max_wal_file_seq_no: 0,
            current_file: None,
        }
    }

    pub fn put(&mut self, wal_record: &WalRecord) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_file.is_none() {
            self.current_file = Some(WalFile::create(self.max_wal_file_seq_no)?);
        }

        if let Some(active_wal_file) = self.current_file.as_mut() {
            active_wal_file.append(wal_record);
        }

        Ok(())
    }

    fn get_latest_file(&self) -> Result<WalFile, Box<dyn std::error::Error>> {
        let mut latest_wal_file: Option<WalFile> = None;
        let wal_files = self.list_files()?;

        for wal_file in wal_files {
            if latest_wal_file
                .as_ref()
                .is_none_or(|latest| wal_file.header.seq_no > latest.header.seq_no)
            {
                latest_wal_file = Some(wal_file);
            }
        }

        latest_wal_file.ok_or_else(|| "No WAL files found".into())
    }

    fn read_all(&self) -> Vec<WalRecord> {
        Vec::new()
    }

    fn list_files(&self) -> Result<Vec<WalFile>, Box<dyn std::error::Error>> {
        let mut wal_files = Vec::new();

        for entry in fs::read_dir("./")? {
            let path = entry?.path();

            if !path.extension().is_some_and(|ext| ext == "wal") {
                continue;
            }

            let Some(path_str) = path.to_str() else {
                continue;
            };

            let wal_file = WalFile::from_path(path_str)?;
            wal_files.push(wal_file);
        }

        Ok(wal_files)
    }
}
