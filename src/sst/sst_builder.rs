use std::{fs::OpenOptions, io::Write, path::Path};

use crate::{
    mem_table::MemTable,
    sst::{block_builder::SSTBlock, bloom_filter::BloomFilter},
    types::KeyValue,
};

pub struct SSTBuilder {
    mem_table: Box<dyn MemTable>,
    bloom_filter: BloomFilter,
    blocks: Vec<SSTBlock>,
    index_no: u32,
}

impl SSTBuilder {
    pub fn new(index_no: u32, mem_table: Box<dyn MemTable>) -> Self {
        let num_elems = mem_table.size() as u64;

        let mut bloom_filter = BloomFilter::new(num_elems, 0.1);

        SSTBuilder {
            index_no,
            mem_table,
            bloom_filter,
            blocks: Vec::new(),
        }
    }

    pub fn build(&mut self) {
        let mut sst_block = SSTBlock::new();

        for (key, value) in self.mem_table.get_all_keys() {
            self.bloom_filter.add(&key);

            let key_val = &KeyValue { key, value };
            if !sst_block.add(key_val) {
                self.blocks.push(sst_block);
                sst_block = SSTBlock::new();
                sst_block.add(key_val);
            }
        }
    }

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new("./sst");

        let mut file = OpenOptions::new().write(true).append(true).open(path)?;

        for block in &self.blocks {
            file.write_all(&block.to_bytes())?;
        }

        file.write_all(&self.bloom_filter.to_bytes())?;

        Ok(())
    }
}
