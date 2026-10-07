use std::io;

use crate::{
    mem_table::{MemTable, btree_mem_table::BTreeMemTable},
    sst::sst_manager::{self, SSTManager, SSTOptions},
    types::KeyValue,
    wal::{
        wal_manager::{WalManager, WalOptions},
        wal_record::{OperationTypeEnum, WalError, WalRecord},
    },
};

#[derive(Debug)]
pub enum StoreError {
    WalError(WalError),
    IoError(io::Error),
    Message(String),
    Other(Box<dyn std::error::Error>),
}

impl From<WalError> for StoreError {
    fn from(err: WalError) -> Self {
        StoreError::WalError(err)
    }
}

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> Self {
        StoreError::IoError(err)
    }
}

impl From<&str> for StoreError {
    fn from(err: &str) -> Self {
        StoreError::Message(err.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for StoreError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        StoreError::Other(err)
    }
}

pub struct StorageOptions {
    /* WAL file options */
    pub wal_options: WalOptions,

    /* WAL file options */
    pub sst_options: SSTOptions,
}

pub struct Storage<'a> {
    options: &'a StorageOptions,

    term: u32,
    index: u32,

    wal_manager: WalManager<'a>,
    sst_manager: SSTManager<'a>,

    mem_table: Box<dyn MemTable>,

    immutable_mem_tables: Vec<Box<dyn MemTable>>,
}

impl<'a> Storage<'a> {
    pub fn new(options: &'a StorageOptions) -> Result<Self, StoreError> {
        let wal_manager = WalManager::new(&options.wal_options)?;
        let sst_manager = SSTManager::new(&options.sst_options);

        let mem_table = Box::new(BTreeMemTable::new());

        Ok(Self {
            options,
            term: 0,
            index: 0,
            wal_manager,
            sst_manager,
            mem_table,
            immutable_mem_tables: Vec::new(),
        })
    }

    pub fn put(&mut self, key: String, value: String) -> Result<(), StoreError> {
        let current_index = self.index;
        self.index += 1;

        self.wal_manager.write(&WalRecord {
            term: self.term,
            index: current_index,
            op_type: OperationTypeEnum::INSERT,
            key_val: KeyValue {
                key: key.clone().into_bytes(),
                value: Some(value.clone().into_bytes()),
            },
        })?;

        self.mem_table.put(key.into_bytes(), value.into_bytes());

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let val = self.mem_table.get(key.as_bytes())?;
        String::from_utf8(val).ok()
    }

    pub fn delete(&mut self, key: String) -> Result<(), StoreError> {
        let current_index = self.index;
        self.index += 1;

        self.mem_table.delete(key.as_bytes());

        self.wal_manager.write(&WalRecord {
            term: self.term,
            index: current_index,
            op_type: OperationTypeEnum::DELETE,
            key_val: KeyValue {
                key: key.into_bytes(),
                value: None,
            },
        })?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), StoreError> {
        let old_sst = std::mem::replace(&mut self.mem_table, Box::new(BTreeMemTable::new()));
        self.mem_table = Box::new(BTreeMemTable::new());

        self.sst_manager.flush(old_sst)?;

        Ok(())
    }
}
