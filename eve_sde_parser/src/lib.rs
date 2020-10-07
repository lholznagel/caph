mod parser;

pub use self::parser::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ParserResult, EveSdeParserError> {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    parser::EveSdeParser::parse(&mut reader)
}

pub fn from_reader<R: parser::ByteReader>(
    reader: &mut R,
) -> Result<ParserResult, EveSdeParserError> {
    parser::EveSdeParser::parse(reader)
}
