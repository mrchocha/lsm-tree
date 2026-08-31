use crate::wal::bytes_reader::{ByteReader, ByteReaderError};
use std::error::Error;

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
    pub fn to_bytes(self) -> Vec<u8> {
        let team_bytes = self.term.to_be_bytes().to_vec();
        let id_bytes = self.id.to_be_bytes().to_vec();
        let op_type_bytes = self.op_type as u8;
        let key_bytes = self.key.as_bytes().to_vec();
        let val_bytes = self.value.as_bytes().to_vec();

        let key_len = key_bytes.len().to_be_bytes().to_vec();
        let val_len = val_bytes.len().to_be_bytes().to_vec();

        let bytes_rep_arr = [
            team_bytes,
            id_bytes,
            vec![op_type_bytes],
            key_len,
            val_len,
            key_bytes,
            val_bytes,
        ];

        let bytes_rep = bytes_rep_arr.concat();

        bytes_rep
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
