mod blueprint;
mod category_ids;
mod group_ids;
mod schematic;
mod station;
mod type_ids;
mod type_material;
mod unique_name;

use crate::error::*;
use crate::reader::*;

pub use self::blueprint::*;
pub use self::category_ids::*;
pub use self::group_ids::*;
pub use self::schematic::*;
pub use self::station::*;
pub use self::type_ids::*;
pub use self::type_material::*;
pub use self::unique_name::*;

use std::collections::HashMap;

/// <https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html>
#[derive(Debug)]
pub struct EveSdeParser;

impl EveSdeParser {
    pub fn parse<R: ByteReader>(
        reader: &mut R,
        requests: Vec<ParseRequest>,
    ) -> Result<Vec<ParseResult>> {
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
                if filename.contains(&x.path()) {
                    match x {
                        ParseRequest::Blueprints => results.push(ParseResult::Blueprints(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::CategoryIds => results.push(ParseResult::CategoryIds(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::GroupIds => results.push(ParseResult::GroupIds(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::Schematics => results.push(ParseResult::Schematic(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::Stations => results.push(ParseResult::Stations(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::TypeIds => results
                            .push(ParseResult::TypeIds(serde_yaml::from_slice(&data).unwrap())),
                        ParseRequest::TypeMaterials => results.push(ParseResult::TypeMaterials(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                        ParseRequest::UniqueNames => results.push(ParseResult::UniqueNames(
                            serde_yaml::from_slice(&data).unwrap(),
                        )),
                    };
                }
            }
        }

        Ok(results)
    }
}

#[derive(Clone)]
pub enum ParseResult {
    Blueprints(HashMap<u32, Blueprint>),
    CategoryIds(HashMap<u32, CategoryIds>),
    GroupIds(HashMap<u32, GroupIds>),
    Schematic(HashMap<u32, Schematic>),
    TypeIds(HashMap<u32, TypeIds>),
    TypeMaterials(HashMap<u32, TypeMaterial>),
    UniqueNames(Vec<UniqueName>),
    Stations(Vec<Station>),
}

pub enum ParseRequest {
    Blueprints,
    CategoryIds,
    GroupIds,
    Schematics,
    Stations,
    TypeIds,
    TypeMaterials,
    UniqueNames,
}

impl ParseRequest {
    pub fn path(&self) -> String {
        match self {
            Self::Blueprints    => "sde/fsd/blueprints.yaml".into(),
            Self::CategoryIds   => "sde/fsd/categoryIDs.yaml".into(),
            Self::GroupIds      => "sde/fsd/groupIDs.yaml".into(),
            Self::Schematics    => "sde/fsd/planetSchematics.yaml".into(),
            Self::Stations      => "sde/bsd/staStations.yaml".into(),
            Self::TypeIds       => "sde/fsd/typeIDs.yaml".into(),
            Self::TypeMaterials => "sde/fsd/typeMaterials.yaml".into(),
            Self::UniqueNames   => "sde/bsd/invUniqueNames".into(),
        }
    }
}
