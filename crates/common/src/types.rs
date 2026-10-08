use crate::bytes_reader::{ByteReader, ByteReaderError};

pub struct KeyValue {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

impl KeyValue {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bin_arr = Vec::new();

        bin_arr.extend_from_slice(&self.key.len().to_be_bytes());
        bin_arr.extend_from_slice(&self.key);

        if let Some(val) = &self.value {
            bin_arr.extend_from_slice(&val.len().to_be_bytes());
            bin_arr.extend_from_slice(val);
        } else {
            let empty_val_size: usize = 0;
            bin_arr.extend_from_slice(&empty_val_size.to_be_bytes());
        }

        bin_arr
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ByteReaderError> {
        let mut breader = ByteReader::new(bytes);
        let key_val = breader.read_key_val()?;

        Ok(key_val)
    }
}
