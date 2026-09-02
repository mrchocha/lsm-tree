mod mem_table;
mod storage;
mod wal;

fn main() {
    let wal_record = wal::wal_record::WalRecord {
        term: 1,
        index: 1,
        op_type: wal::wal_record::OperationTypeEnum::INSERT,
        key: "name".to_string(),
        value: "rahul".to_string(),
    };

    let byt = wal_record.to_bytes();
    let val = wal::wal_record::WalRecord::from_bytes(&byt);
    println!("Hello, world! ");
}
