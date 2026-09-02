mod btree_mem_table;

pub struct KeyValue {
    pub key: String,
    pub value: String,
}

pub trait MemTable {
    fn put(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<String>;
    fn delete(&mut self, key: String);
    fn scan(&self, start: String, end: String) -> Vec<KeyValue>;
}
