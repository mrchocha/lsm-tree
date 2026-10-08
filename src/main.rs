use crate::{
    sst::sst_manager::SSTOptions,
    storage::{Storage, StorageOptions},
};

use common::{bytes_reader, types};
use wal;

mod mem_table;
mod sst;
mod storage;

fn main() {
    let options = &StorageOptions {
        wal_options: wal::manager::WalOptions {
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
