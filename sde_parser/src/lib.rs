mod parser;

use self::parser::{EveSdeParserError, TypeIdData};

use eve_online_api::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<TypeId, TypeIdData>, EveSdeParserError> {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    parser::EveSdeParser::parse(&mut reader)
}

pub fn from_reader<R: parser::ByteReader>(reader: &mut R) -> Result<HashMap<TypeId, TypeIdData>, EveSdeParserError> {
    parser::EveSdeParser::parse(reader)
}