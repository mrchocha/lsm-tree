use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    mem_table::MemTable,
    sst::{block_builder::SSTBlock, bloom_filter::BloomFilter},
    types::KeyValue,
};

/*
SSTable Structure
-------------------------------------
| Block (1) _________________________|
| Block (2) _________________________|
| Block (3) _________________________|
|....................................|
|block indices u64 x N...............|
|bloom filter........................|
|block indices position..............|
|bloom filter position...............|
|other footer........................|
*/
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

        self.blocks.push(sst_block);
    }

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{0}_data.sst", self.index_no);
        let path = PathBuf::from("./sst").join(&file_name);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)?;

        let mut index = 0 as usize;

        let mut block_indices: Vec<[u8; 8]> = Vec::new();
        let mut sst_indices: Vec<[u8; 8]> = Vec::new();

        for block in &self.blocks {
            let bytes = &block.to_bytes();
            index += bytes.len();
            file.write_all(bytes)?;
            block_indices.push(index.to_be_bytes());
        }

        // block index start position
        sst_indices.push(index.to_be_bytes());

        for block_index in block_indices {
            file.write_all(&block_index)?;
            index += 8;
        }

        // bloom filter stat index
        sst_indices.push(index.to_be_bytes());

        let bloom_filter_bytes = &self.bloom_filter.to_bytes();
        file.write_all(bloom_filter_bytes)?;
        index += bloom_filter_bytes.len();

        for sst_index in sst_indices {
            file.write_all(&sst_index)?;
        }
        Ok(())
    }
}
