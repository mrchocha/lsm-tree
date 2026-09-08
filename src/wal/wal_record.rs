use crate::{
    bytes_reader::{ByteReader, ByteReaderError},
    types::KeyValue,
};

#[derive(Debug)]
pub enum WalError {
    InvalidOperationType(u8),
    ByteReaderError(ByteReaderError),
}

impl From<ByteReaderError> for WalError {
    fn from(err: ByteReaderError) -> Self {
        WalError::ByteReaderError(err)
    }
}

#[derive(Clone)]
pub enum OperationTypeEnum {
    INSERT = 0,
    UPDATE,
    DELETE,
}

impl OperationTypeEnum {
    fn from_u8(val: u8) -> Result<Self, WalError> {
        match val {
            0 => Ok(OperationTypeEnum::INSERT),
            1 => Ok(OperationTypeEnum::UPDATE),
            2 => Ok(OperationTypeEnum::DELETE),
            _ => Err(WalError::InvalidOperationType(val)),
        }
    }
}

pub struct WalRecord {
    pub term: u32,
    pub index: u32,
    pub op_type: OperationTypeEnum,
    pub key_val: KeyValue,
}

impl WalRecord {
    pub fn to_bytes(&self) -> Vec<u8> {
        let key_val_bytes = self.key_val.to_bytes();

        let mut bytes = Vec::with_capacity(4 + 4 + 1 + key_val_bytes.len());

        bytes.extend_from_slice(&self.term.to_be_bytes());
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes.push(self.op_type.clone() as u8);
        bytes.extend_from_slice(&key_val_bytes);

        bytes
    }

    pub fn checksum(bytes: &[u8]) -> u32 {
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let checksum: u32 = crc.checksum(&bytes);

        checksum
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WalError> {
        let mut breader = ByteReader::new(bytes);

        let term = breader.read_u32()?;
        let index = breader.read_u32()?;
        let op_type = OperationTypeEnum::from_u8(breader.read_u8()?)?;
        let key_val = breader.read_key_val()?;

        Ok(WalRecord {
            term,
            index,
            op_type,
            key_val,
        })
    }
}
