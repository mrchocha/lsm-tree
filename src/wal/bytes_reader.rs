use std::fmt::Error;

#[derive(Debug)]
pub enum ByteReaderError {
    InvalidData,
    InvalidUtf8,
}

pub struct ByteReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub fn read_u32(&mut self) -> Result<u32, ByteReaderError> {
        let end = self
            .position
            .checked_add(4)
            .ok_or(ByteReaderError::InvalidData)?;

        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ByteReaderError::InvalidData)?;

        self.position = end;

        Ok(u32::from_be_bytes(
            bytes.try_into().map_err(|_| ByteReaderError::InvalidData)?,
        ))
    }

    pub fn read_u8(&mut self) -> Result<u8, ByteReaderError> {
        let end = self
            .position
            .checked_add(1)
            .ok_or(ByteReaderError::InvalidData)?;

        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ByteReaderError::InvalidData)?;

        self.position = end;

        Ok(u8::from_be_bytes(
            bytes.try_into().map_err(|_| ByteReaderError::InvalidData)?,
        ))
    }

    pub fn read_usize(&mut self) -> Result<usize, ByteReaderError> {
        let end = self
            .position
            .checked_add(8)
            .ok_or(ByteReaderError::InvalidData)?;

        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ByteReaderError::InvalidData)?;

        self.position = end;

        Ok(usize::from_be_bytes(
            bytes.try_into().map_err(|_| ByteReaderError::InvalidData)?,
        ))
    }

    pub fn read_str(&mut self, len: usize) -> Result<String, ByteReaderError> {
        let end = self
            .position
            .checked_add(len)
            .ok_or(ByteReaderError::InvalidData)?;

        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or(ByteReaderError::InvalidData)?;

        self.position = end;

        String::from_utf8(bytes.to_vec()).map_err(|_| ByteReaderError::InvalidUtf8)
    }
}
