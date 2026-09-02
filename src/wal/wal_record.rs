use crate::wal::bytes_reader::{ByteReader, ByteReaderError};

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
    pub id: u32,
    pub op_type: OperationTypeEnum,
    pub key: String,
    pub value: String,
}

impl WalRecord {
    pub fn to_bytes(&self) -> Vec<u8> {
        let key_bytes = self.key.as_bytes();
        let value_bytes = self.value.as_bytes();

        let mut bytes = Vec::with_capacity(4 + 4 + 1 + 8 + 8 + key_bytes.len() + value_bytes.len());

        bytes.extend_from_slice(&self.term.to_be_bytes());
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.push(self.op_type.clone() as u8);
        bytes.extend_from_slice(&key_bytes.len().to_be_bytes());
        bytes.extend_from_slice(&value_bytes.len().to_be_bytes());
        bytes.extend_from_slice(key_bytes);
        bytes.extend_from_slice(value_bytes);

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
        let id = breader.read_u32()?;
        let op_type = OperationTypeEnum::from_u8(breader.read_u8()?)?;
        let key_len = breader.read_usize()?;
        let val_len = breader.read_usize()?;

        let key = breader.read_str(key_len)?;
        let value = breader.read_str(val_len)?;

        Ok(WalRecord {
            term,
            id,
            op_type,
            key,
            value,
        })
    }
}
