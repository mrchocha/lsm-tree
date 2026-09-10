use crate::{
    mem_table::MemTable,
    types::KeyValue,
    wal::{
        wal_file,
        wal_manager::WalManager,
        wal_record::{OperationTypeEnum, WalRecord},
    },
};

pub struct StorageOptions {
    /* WAL file options */
    pub wal_file_path: String,
}

pub struct Storage<'a> {
    options: &'a StorageOptions,

    term: u32,
    index: u32,

    wal_manager: WalManager<'a>,
    mem_table: Box<dyn MemTable>,
}

impl<'a> Storage<'a> {
    pub fn new(
        options: &'a StorageOptions,
        mem_table: Box<dyn MemTable>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let wal_manager = WalManager::new_with_latest_file(options)?;

        Ok(Self {
            options,
            term: 0,
            index: 0,
            wal_manager,
            mem_table,
        })
    }

    pub fn put(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
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

    fn get(&self, key: &str) -> Option<String> {
        self.mem_table.get(key.as_bytes())
    }

    fn delete(&mut self, key: String) -> Result<(), Box<dyn std::error::Error>> {
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
}
