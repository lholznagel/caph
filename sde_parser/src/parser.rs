mod error;
mod reader;
mod type_ids;

use self::error::Result;

pub use self::reader::ByteReader;
pub use self::error::EveSdeParserError;
pub use self::type_ids::TypeIdData;

use eve_online_api::TypeId;
use std::collections::HashMap;

/// https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
#[derive(Debug)]
pub struct EveSdeParser;

impl EveSdeParser {
    pub fn parse<R: reader::ByteReader>(reader: &mut R) -> Result<HashMap<TypeId, TypeIdData>> {
        let type_data;

        loop {
            if reader.read_u32be()? != 0x50_4b_03_04 {
                return Err(EveSdeParserError::InvalidFileFormat)
            }

            // Skip version
            reader.read_u16le()?;
            // Skip flags
            reader.read_u16le()?;

            if reader.read_u16be()? != 0x08_00 {
                return Err(EveSdeParserError::InvalidCompression);
            }

            // Skip mod time
            reader.read_u16le()?;
            // Skip mod date
            reader.read_u16le()?;
            // Skip crc
            reader.read_u32le()?;
            // Skip compressed size
            let data_length = reader.read_u32le()?;
            // Skip uncompressed size
            reader.read_u32le()?;

            let file_name_length = reader.read_u16le()?;
            // Skip extra field len
            reader.read_u16le()?;

            let filename = reader.read_length(file_name_length as usize)?;
            let filename = String::from_utf8(filename)?;

            if filename != "sde/fsd/typeIDs.yaml" {
                reader.skip(data_length as usize)?;
                continue;
            } else {
                let data = reader.read_length(data_length as usize)?;
                let data = miniz_oxide::inflate::decompress_to_vec(&data).unwrap();
                type_data = serde_yaml::from_slice(&data).unwrap();
                break;
            }
        }

        Ok(type_data)
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    use std::io::Cursor;

    #[test]
    fn valid_header() {
        assert!(EveSdeParser::parse(&mut Cursor::new(vec![
            0x50, 0x4b, 0x03, 0x04,             // File format
            0x00, 0x00,                         // Version
            0x00, 0x00,                         // Flags
            0x08, 0x00                          // Compression
        ])).is_ok());
    }

    #[test]
    fn invlid_header() {
        assert!(EveSdeParser::parse(&mut Cursor::new(vec![
            0x00, 0x4b, 0x03, 0x04,             // File format
            0x00, 0x00,                         // Version
            0x00, 0x00,                         // Flags
            0x09, 0x00                          // Compression
        ])).is_err());
    }
}