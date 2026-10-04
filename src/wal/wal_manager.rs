use std::fs;

use crate::{
    storage::StorageOptions,
    wal::{wal_file::WalFile, wal_record::WalRecord},
};

pub struct WalOptions {
    pub file_path: String,
    pub max_file_size: usize,
}

pub struct WalManager<'a> {
    options: &'a WalOptions,

    current_file: WalFile,

    reusable_files: Vec<WalFile>,
    max_wal_file_seq_no: u32,
}

impl<'a> WalManager<'a> {
    pub fn new(options: &'a WalOptions) -> Result<WalManager<'a>, Box<dyn std::error::Error>> {
        let mut file = Self::get_latest_file(options)?;
        if file.is_none() {
            file = Some(WalFile::create(options, 1)?);
        }

        let wal_file = file.expect("Wal file not found");

        Ok(WalManager {
            options,
            max_wal_file_seq_no: wal_file.header.seq_no,
            current_file: wal_file,
            reusable_files: Vec::new(),
        })
    }

    fn advance_wal_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.max_wal_file_seq_no += 1;
        let file = WalFile::create(self.options, self.max_wal_file_seq_no)?;

        self.current_file = file;

        Ok(())
    }

    pub fn write(&mut self, wal_record: &WalRecord) -> Result<(), Box<dyn std::error::Error>> {
        let curr_wal_file_size = self.current_file.get_size()?;
        if curr_wal_file_size as usize >= self.options.max_file_size {
            self.advance_wal_file()?
        }

        self.current_file.append(wal_record)?;

        Ok(())
    }

    fn get_latest_file(
        options: &WalOptions,
    ) -> Result<Option<WalFile>, Box<dyn std::error::Error>> {
        let mut latest_wal_file: Option<WalFile> = None;
        let wal_files: Vec<WalFile> = Self::list_files(options)?;

        for wal_file in wal_files {
            if latest_wal_file
                .as_ref()
                .is_none_or(|latest| wal_file.header.seq_no > latest.header.seq_no)
            {
                latest_wal_file = Some(wal_file);
            }
        }

        Ok(latest_wal_file)
    }

    fn read_all(&self) -> Vec<WalRecord> {
        Vec::new()
    }

    fn list_files(options: &WalOptions) -> Result<Vec<WalFile>, Box<dyn std::error::Error>> {
        let mut wal_files = Vec::new();
        let path = options.file_path.clone();

        for entry in fs::read_dir(path)? {
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
