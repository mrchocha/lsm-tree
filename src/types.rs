use crate::bytes_reader::{ByteReader, ByteReaderError};

pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bin_arr = Vec::new();

        bin_arr.extend_from_slice(&self.key.len().to_be_bytes());
        bin_arr.extend_from_slice(&self.key.as_bytes());
        bin_arr.extend_from_slice(&self.value.len().to_be_bytes());
        bin_arr.extend_from_slice(&self.value.as_bytes());

        bin_arr
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ByteReaderError> {
        let mut breader = ByteReader::new(bytes);
        let key_val = breader.read_key_val()?;

        Ok(key_val)
    }
}
