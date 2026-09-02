use crate::{
    mem_table::{KeyValue, MemTable},
    wal::{
        wal_manager::WalManager,
        wal_record::{OperationTypeEnum, WalRecord},
    },
};

struct Storage {
    term: u32,
    index: u32,

    wal_manager: WalManager,
    mem_table: dyn MemTable,
}

impl Storage {
    fn put(&mut self, key: String, value: String) {
        let current_index = self.index;
        self.index += 1;

        self.wal_manager.put(&WalRecord {
            term: self.term,
            index: current_index,
            op_type: OperationTypeEnum::INSERT,
            key: key.clone(),
            value: value.clone(),
        });

        self.mem_table.put(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.mem_table.get(key)
    }
}
