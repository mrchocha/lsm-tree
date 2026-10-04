use crate::{
    sst::sst_manager::SSTOptions,
    storage::{Storage, StorageOptions},
    wal::wal_manager::WalOptions,
};

mod bytes_reader;
mod mem_table;
mod sst;
mod storage;
mod types;
mod wal;

fn main() {
    let options = &StorageOptions {
        wal_options: WalOptions {
            file_path: "./data/wal".to_string(),
            max_file_size: 4069,
        },
        sst_options: SSTOptions {
            file_path: "./data/sst".to_string(),
            max_file_size: 4069,
        },
    };

    let mut storage = Storage::new(options).expect("Error while init storage");

    storage
        .put("Rahul".to_string(), "Ahir".to_string())
        .expect("Error while inserting key");

    storage.flush().expect("error while flush");
    println!("Hello, world! ");
}
