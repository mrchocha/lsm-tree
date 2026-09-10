use crate::{
    mem_table::btree_mem_table::BTreeMemTable,
    sst::sst_builder::SSTBuilder,
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

    let mut storage = Storage::new(options).expect("Error while init storage");

    storage
        .put("Rahul".to_string(), "Ahir".to_string())
        .expect("Error while inserting key");

    storage.flush_sst().expect("error while flush");
    println!("Hello, world! ");
}
