mod error;
mod parser;
mod reader;

pub use self::error::EveSdeParserError;
pub use self::parser::*;

use crate::reader::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn from_file<P: AsRef<Path>>(
    path: P,
    requests: Vec<ParseRequest>,
) -> Result<Vec<ParseResult>, EveSdeParserError> {
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

pub async fn fetch_zip() -> Result<Vec<u8>, ()> {
    let x = reqwest::get("https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec();
    Ok(x)
}
