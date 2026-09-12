use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    bytes_reader::{ByteReader, ByteReaderError, FooterByteReader},
    mem_table::MemTable,
    sst::{
        block_builder::SSTBlock,
        bloom_filter::{self, BloomFilter},
    },
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
|block indices start position........|
|block indices end position..........|
|bloom filter start position.........|
|bloom filter end position...........|
*/
pub struct SSTBuilder {
    bloom_filter: BloomFilter,
    blocks: Vec<SSTBlock>,
}

impl SSTBuilder {
    pub fn new_from_mem_table(mem_table: Box<dyn MemTable>) -> Self {
        let num_elems = mem_table.size() as u64;

        let mut bloom_filter = BloomFilter::new(num_elems, 0.1);
        let mut blocks = Vec::new();

        let mut sst_block = SSTBlock::new();

        for (key, value) in mem_table.get_all_keys() {
            bloom_filter.add(&key);

            let key_val = &KeyValue { key, value };
            if !sst_block.add(key_val) {
                blocks.push(sst_block);
                sst_block = SSTBlock::new();
                sst_block.add(key_val);
            }
        }

        blocks.push(sst_block);

        SSTBuilder {
            bloom_filter,
            blocks,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ByteReaderError> {
        let mut footer_btreader = FooterByteReader::new(bytes);

        let block_bloom_filter_end = footer_btreader.read_u32()? as usize;
        let block_bloom_filter_start = footer_btreader.read_u32()? as usize;

        let block_index_end = footer_btreader.read_u32()? as usize;
        let block_index_start = footer_btreader.read_u32()? as usize;

        let bloom_filter_bytes = footer_btreader
            .read_bytes_vec(block_bloom_filter_end - block_bloom_filter_start + 1)?;

        let bloom_filter = BloomFilter::from_bytes(&bloom_filter_bytes)?;

        let mut blockes_index_bytes = bytes
            .get(block_index_start..block_index_end)
            .ok_or(ByteReaderError::InvalidData)?;

        let mut blockes_bytes = bytes
            .get(0..block_index_start)
            .ok_or(ByteReaderError::InvalidData)?;

        let mut index_btreader = ByteReader::new(blockes_index_bytes);

        let mut block_indices = Vec::new();
        let mut blocks = Vec::new();

        let mut prv_block_index = 0 as usize;

        while !index_btreader.is_finished() {
            let block_index = index_btreader.read_u32()? as usize;
            block_indices.push(block_index);

            let block = SSTBlock::from_bytes(
                bytes
                    .get(prv_block_index..block_index)
                    .ok_or(ByteReaderError::InvalidData)?,
            )?;

            blocks.push(block);

            prv_block_index = block_index
        }

        Ok(Self {
            bloom_filter,
            blocks,
        })
    }

    pub fn flush(&self, index_no: u32) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{0}_data.sst", index_no);
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
        // block index end position
        sst_indices.push(index.to_be_bytes());

        // bloom filter stat position
        sst_indices.push(index.to_be_bytes());

        let bloom_filter_bytes = &self.bloom_filter.to_bytes();
        file.write_all(bloom_filter_bytes)?;
        index += bloom_filter_bytes.len();

        // bloom filter end position
        sst_indices.push(index.to_be_bytes());

        for sst_index in sst_indices {
            file.write_all(&sst_index)?;
        }
        Ok(())
    }
}
