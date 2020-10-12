mod blueprint;
mod type_ids;
mod type_material;
mod solarsystem;

use crate::error::*;
use crate::reader::*;

pub use self::blueprint::*;
pub use self::type_ids::TypeIds;
pub use self::type_material::TypeMaterial;
pub use self::solarsystem::Solarsystem;

use std::collections::HashMap;

/// https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
#[derive(Debug)]
pub struct EveSdeParser;

impl EveSdeParser {
    pub fn parse<R: ByteReader>(reader: &mut R, requests: Vec<ParseRequest>) -> Result<Vec<ParseResult>> {
        let mut results = Vec::new();

        while reader.read_u32be()? == 0x50_4b_03_04 {
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

            let data = reader.read_length(data_length as usize)?;
            let data = miniz_oxide::inflate::decompress_to_vec(&data).unwrap();

            for x in &requests {
                if x.path() == filename {
                    let parsed = match x {
                        ParseRequest::Blueprints => ParseResult::Blueprints(serde_yaml::from_slice(&data).unwrap()),
                        ParseRequest::TypeIds => ParseResult::TypeIds(serde_yaml::from_slice(&data).unwrap()),
                        ParseRequest::TypeMaterials => ParseResult::TypeMaterials(serde_yaml::from_slice(&data).unwrap()),
                    };

                    results.push(parsed);
                }
            }
        }

        Ok(results)
    }
}

pub enum ParseResult {
    Blueprints(HashMap<u32, Blueprint>),
    TypeIds(HashMap<u32, TypeIds>),
    TypeMaterials(HashMap<u32, TypeMaterial>)
}

pub enum ParseRequest {
    Blueprints,
    TypeIds,
    TypeMaterials
}

impl ParseRequest {
    pub fn path(&self) -> String {
        match self {
            Self::Blueprints => "sde/fsd/blueprints.yaml".into(),
            Self::TypeIds => "sde/fsd/typeIDs.yaml".into(),
            Self::TypeMaterials => "sde/fsd/typeMaterials.yaml".into(),
        }
    }
}