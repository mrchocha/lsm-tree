fn main() {
    let options = &storage::engine::StorageOptions {
        wal_options: wal::manager::WalOptions {
            file_path: "./data/wal".to_string(),
            max_file_size: 4069,
        },
        sst_options: sst::manager::SSTOptions {
            file_path: "./data/sst".to_string(),
            max_file_size: 4069,
        },
    };

    let mut storage = storage::engine::Storage::new(options).expect("Error while init storage");

    storage
        .put("Rahul".to_string(), "Ahir".to_string())
        .expect("Error while inserting key");

    storage.flush().expect("error while flush");
    println!("Hello, world! ");
}
