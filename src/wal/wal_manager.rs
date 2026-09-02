use crate::wal::wal_record::WalRecord;

pub struct WalManager {}

impl WalManager {
    fn new() -> Self {
        WalManager {}
    }

    fn append(wal_record: &WalRecord) {}

    fn read_all() -> Vec<WalRecord> {
        Vec::new()
    }

    fn list_files() {}
}
