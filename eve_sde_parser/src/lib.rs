mod error;
mod parser;
mod reader;

pub use self::parser::*;
pub use self::error::EveSdeParserError;

use crate::reader::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn from_file<P: AsRef<Path>>(path: P, requests: Vec<ParseRequest>) -> Result<Vec<ParseResult>, EveSdeParserError> {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    parser::EveSdeParser::parse(&mut reader, requests)
}

pub fn from_reader<R: ByteReader>(
    reader: &mut R,
    requests: Vec<ParseRequest>,
) -> Result<Vec<ParseResult>, EveSdeParserError> {

    parser::EveSdeParser::parse(reader, requests)
}
