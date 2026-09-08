use crate::{
    mem_table::btree_mem_table::BTreeMemTable,
    storage::{Storage, StorageOptions},
};

mod bytes_reader;
mod mem_table;
mod sst;
mod storage;
mod types;
mod wal;

fn main() {
    let options = &StorageOptions {
        wal_file_path: "./wal".to_string(),
    };

    let btree_mem_table = Box::new(BTreeMemTable::new());
    let mut storage = Storage::new(options, btree_mem_table).expect("Error while init storage");

    storage
        .put("Rahul".to_string(), "Ahir".to_string())
        .expect("Error while inserting key");
    println!("Hello, world! ");
}
