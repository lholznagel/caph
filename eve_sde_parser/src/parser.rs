mod error;
mod reader;
mod result;
mod type_data;
mod type_material;

use self::error::Result;

pub use self::error::EveSdeParserError;
pub use self::reader::ByteReader;
pub use self::result::ParserResult;
pub use self::type_data::TypeIdData;
pub use self::type_material::TypeMaterial;

use std::collections::HashMap;

/// https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
#[derive(Debug)]
pub struct EveSdeParser;

impl EveSdeParser {
    pub fn parse<R: reader::ByteReader>(reader: &mut R) -> Result<ParserResult> {
        let materials;
        let mut type_data = HashMap::new();

        loop {
            if reader.read_u32be()? != 0x50_4b_03_04 {
                return Err(EveSdeParserError::InvalidFileFormat);
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

            if filename == "sde/fsd/typeIDs.yaml" {
                let data = reader.read_length(data_length as usize)?;
                let data = miniz_oxide::inflate::decompress_to_vec(&data).unwrap();
                type_data = serde_yaml::from_slice(&data).unwrap();
            } else if filename == "sde/fsd/typeMaterials.yaml" {
                let data = reader.read_length(data_length as usize)?;
                let data = miniz_oxide::inflate::decompress_to_vec(&data).unwrap();
                materials = serde_yaml::from_slice(&data).unwrap();
                break;
            } else {
                reader.skip(data_length as usize)?;
                continue;
            }
        }

        Ok(ParserResult {
            type_data,
            materials,
        })
    }
}
