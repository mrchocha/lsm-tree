use std::{collections::BTreeMap, sync::RwLock};

use crate::{mem_table::MemTable, types::KeyValue};

pub struct BTreeMemTable {
    store: RwLock<BTreeMap<Vec<u8>, Option<Vec<u8>>>>,
}

impl BTreeMemTable {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(BTreeMap::new()),
        }
    }
}

impl MemTable for BTreeMemTable {
    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let mut store = self.store.write().unwrap();

        store.insert(key, Some(value));
    }

    fn get(&self, key: &[u8]) -> Option<String> {
        let store = self.store.read().unwrap();

        let value = store.get(key)?.as_ref()?;

        String::from_utf8(value.clone()).ok()
    }

    fn delete(&mut self, key: &[u8]) {
        let mut store = self.store.write().unwrap();

        store.remove(key);
    }

    // fn scan(&self, start: String, end: String) -> Vec<KeyValue> {
    //     let store = self.store.read().unwrap();

    //     let start_bytes = start.into_bytes();
    //     let end_bytes = end.into_bytes();

    //     let mut kv_vect = Vec::new();

    //     for (key_bytes, opt_value_bytes) in store.range(start_bytes..=end_bytes) {
    //         if let Some(value_bytes) = opt_value_bytes {
    //             let key = String::from_utf8(key_bytes.clone()).expect("key decode error");
    //             let value = String::from_utf8(value_bytes.clone()).expect("value decode error");
    //             kv_vect.push(KeyValue { key, value });
    //         }
    //     }

    //     kv_vect
    // }

    fn size(&self) -> usize {
        let store = self.store.read().unwrap();

        store.len()
    }

    fn get_all_keys(&self) -> Vec<(Vec<u8>, Option<Vec<u8>>)> {
        let mut key_vals = Vec::new();

        let store = self.store.read().unwrap();

        for (key, val) in store.iter() {
            key_vals.push((key.to_owned(), val.to_owned()));
        }

        key_vals
    }
}
