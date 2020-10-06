use crate::parser::error::*;

pub trait ByteReader: std::io::Read {
    /// Reads one byte from the byte array and returns it as u8 value
    ///
    /// If there are any errors, a `ProtocolError` is returned
    /// containing the underlying `std::io::Error`
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buffer = [0; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    /// Reads two byte from the byte array and returns it as u16 value
    ///
    /// If there are any errors, a `ProtocolError` is returned
    /// containing the underlying `std::io::Error`
    #[inline]
    fn read_u16be(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    #[inline]
    fn read_u16le(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    /// Reads four byte from the byte array and returns it as u16 value
    ///
    /// If there are any errors, a `ProtocolError` is returned
    /// containing the underlying `std::io::Error`
    #[inline]
    fn read_u32be(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    #[inline]
    fn read_u32le(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Reads the given amount of bytes and returns them as a `Vec<u8>`
    ///
    /// If there are any errors, a `ProtocolError` is returned
    /// containing the underlying `std::io::Error`
    #[inline]
    fn read_length(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(length);

        for _ in 0..length {
            buf.push(0);
        }

        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    #[inline]
    fn skip(&mut self, length: usize) -> Result<()> {
        self.read_length(length).map(|_| ())
    }
}

impl<R: std::io::Read + ?Sized> ByteReader for R {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn read_u8_001() {
        let mut reader = Cursor::new(vec![100u8]);
        assert_eq!(reader.read_u8().unwrap(), 100u8);
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn read_u8_002() {
        let mut reader = Cursor::new(vec![100u8, 100u8]);
        assert_eq!(reader.read_u8().unwrap(), 100u8);
        assert_eq!(reader.read_u8().unwrap(), 100u8);
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn read_u8_003() {
        let mut reader = Cursor::new(Vec::new());
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn read_u16be_001() {
        let mut reader = Cursor::new(vec![100u8, 100u8]);
        assert_eq!(reader.read_u16be().unwrap(), 25700u16);
        assert!(reader.read_u16be().is_err());
    }

    #[test]
    fn read_u16be_002() {
        let mut reader = Cursor::new(vec![100u8, 100u8, 100u8, 100u8]);
        assert_eq!(reader.read_u16be().unwrap(), 25700u16);
        assert_eq!(reader.read_u16be().unwrap(), 25700u16);
        assert!(reader.read_u16be().is_err());
    }

    #[test]
    fn read_u16be_003() {
        let mut reader = Cursor::new(Vec::new());
        assert!(reader.read_u16be().is_err());
    }

    #[test]
    fn read_length_001() {
        let mut reader = Cursor::new(vec![100u8, 100u8]);
        assert_eq!(reader.read_length(2usize).unwrap(), vec![100u8, 100u8]);
        assert!(reader.read_length(2usize).is_err());
    }

    #[test]
    fn read_length_002() {
        let mut reader = Cursor::new(vec![100u8, 100u8, 100u8, 100u8]);
        assert_eq!(reader.read_length(2usize).unwrap(), vec![100u8, 100u8]);
        assert_eq!(reader.read_length(2usize).unwrap(), vec![100u8, 100u8]);
        assert!(reader.read_length(2usize).is_err());
    }

    #[test]
    fn read_length_003() {
        let mut reader = Cursor::new(Vec::new());
        assert!(reader.read_length(2usize).is_err());
    }
}