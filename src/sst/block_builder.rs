use core::num;
use std::vec;

use crate::{
    bytes_reader::{ByteReader, ByteReaderError},
    types::KeyValue,
};

const MAX_BLOCK_SIZE: usize = 4069;
const RESTART_INTERVAL: u32 = 16;

/*
SSTBlock Structure (4Kb) (best effort)
-------------------------------------
| KVBinary (1)_______________________|
| KVBinary (2)_______________________|
| KVBinary (3)_______________________|
|....................................|
|restart_offsets.....................|
|num_restart_offsets.................|
|..............crc check.............|
*/

struct SSTBlock {
    binaries: Vec<u8>,

    restart_offsets: Vec<usize>,
    num_restart: u32,
    num_records: u32,
}

impl SSTBlock {
    pub fn new() -> Self {
        SSTBlock {
            binaries: Vec::new(),
            restart_offsets: Vec::new(),
            num_restart: 0,
            num_records: 0,
        }
    }

    pub fn add(&mut self, key_val: &KeyValue) -> bool {
        let key_val_binaries = key_val.to_bytes();
        let should_restart = self.num_records % RESTART_INTERVAL == 0;
        let new_num_restart = self.num_restart + should_restart as u32;

        let new_length =
            self.binaries.len() + key_val_binaries.len() + (new_num_restart as usize) * 4 + 4;

        // block length should not exit MAX_BLOCK_SIZE (best effort)
        if new_length > MAX_BLOCK_SIZE {
            return false;
        }

        // note offset at reset point
        if should_restart {
            self.restart_offsets.push(self.binaries.len());
            self.num_restart += 1;
        }

        self.binaries.extend_from_slice(&key_val_binaries);
        self.num_records += 1;

        return true;
    }

    pub fn to_binary(&self) -> Vec<u8> {
        let mut bin_arr = Vec::new();

        bin_arr.extend_from_slice(&self.binaries);

        for offset in &self.restart_offsets {
            bin_arr.extend_from_slice(&offset.to_be_bytes());
        }

        bin_arr.extend_from_slice(&self.num_restart.to_be_bytes());

        bin_arr
    }

    pub fn from_binary(bytes: &[u8]) -> Result<Self, ByteReaderError> {
        let num_restart_bytes = &bytes[bytes.len() - 4..];

        let mut breader = ByteReader::new(num_restart_bytes);
        let num_restart = breader.read_u32()?;

        let restart_start = bytes.len() - 4 - (num_restart as usize * 4);

        let mut restart_offsets = Vec::new();
        for i in 0..num_restart {
            let offset_position = restart_start + (i as usize) * 4;
            let offset_bytes = &bytes[offset_position..(offset_position + 4)];

            let mut reset_breader = ByteReader::new(offset_bytes);
            restart_offsets.push(reset_breader.read_usize()?);
        }

        let binaries = bytes[..restart_start].to_vec();

        Ok(SSTBlock {
            binaries,
            restart_offsets,
            num_restart,
            num_records: 0,
        })
    }
}
