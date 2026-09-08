use crate::types::KeyValue;

const MAX_BLOCK_SIZE: u32 = 4069;
const RESTART_INTERVAL: u32 = 16;

struct SSTBlock {
    restart_offsets: Vec<u32>,
    num_restart: u32,
    num_records: u32,

    binaries: Vec<u8>,
}

impl SSTBlock {
    pub fn add(&mut self, key_val: &KeyValue) {
        self.binaries.extend_from_slice(&key_val.to_bytes());
        self.num_records += 1;

        if self.num_restart == 0 {
            self.restart_offsets.push(0);
            self.num_restart += 1;

            return;
        }

        if let Some(&last_offset) = self.restart_offsets.last()
            && self.num_records - last_offset >= 16
        {
            self.restart_offsets.push(self.num_records);
            self.num_restart += 1;
        }
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
}
