use xxhash_rust::xxh64::xxh64;
pub struct BloomFilter {
    bit_size: u64,
    num_hash: u64,
    bit_arr: Vec<u8>,
}

impl BloomFilter {
    pub fn new(num_elems: u64, acceptable_fp: f64) -> Self {
        let ln2 = 2.0_f64.ln();
        let bit_size = ((num_elems as f64 * acceptable_fp.ln()) / ln2.powi(2)) as u64;
        let num_hash = (bit_size / num_elems) * ln2 as u64;

        let bit_arr = Vec::with_capacity(bit_size as usize);

        BloomFilter {
            bit_arr,
            bit_size,
            num_hash,
        }
    }

    fn hashes(&self, key: &[u8]) -> Vec<u64> {
        let mut indices = Vec::new();

        let hash_1 = xxh64(key, 0);
        let hash_2 = xxh64(key, 10);

        for i in 0..self.num_hash {
            let position = (hash_1 + i * hash_2) % (self.bit_size);
            indices.push(position);
        }

        indices
    }

    pub fn add(&mut self, key: &[u8]) {
        for index in self.hashes(key) {
            self.bit_arr.insert(index as usize, 1);
        }
    }

    pub fn is_present(&self, key: &[u8]) -> bool {
        for index in self.hashes(key) {
            if let Some(data) = self.bit_arr.get(index as usize)
                && *data == 0
            {
                return false;
            }
        }
        return true;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut binary = Vec::new();
        binary.extend_from_slice(&self.num_hash.to_be_bytes());
        binary.extend_from_slice(&self.bit_size.to_be_bytes());
        binary.extend_from_slice(&self.bit_arr);
        binary
    }
}
